use alloc::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

const HEADER_XSRF_NAME: &str = "X-Atlassian-Token";
const HEADER_XSRF_SENT: &str = "request.header(\"X-Atlassian-Token\", \"no-check\");";

const METHODS: [(&str, &str); 5] =
    [("delete", "Delete"), ("get", "Get"), ("patch", "Patch"), ("post", "Post"), ("put", "Put")];

const MULTIPART: &str = "multipart/form-data";
const OPERATION_COUNT_TOTAL: u64 = 397;
const PRODUCTS: [(&str, &str); 3] = [("automation", ""), ("v1", ""), ("v2", "/wiki/api/v2")];

#[derive(Debug)]
struct Generated {
    file: PathBuf,
    text: String,
}

#[derive(Clone, Copy, Debug)]
struct Specified<'document> {
    document: &'document Value,
    item: &'document Value,
    operation: &'document Value,
    path: &'document str,
    variant: &'static str,
}

impl<'document> Specified<'document> {
    fn resolved(&self, node: &'document Value) -> &'document Value {
        assert!(self.document.is_object(), "a specification is an object");

        let Some(reference) = node.get("$ref").and_then(Value::as_str) else {
            return node;
        };

        let target = reference.strip_prefix('#').and_then(|pointer| self.document.pointer(pointer));

        target.expect("a reference names a node of its own specification")
    }
}

pub(crate) fn document_read(module: &str) -> Value {
    assert!(!module.is_empty(), "a specification is named for its module");

    let path = root().join("spec").join(format!("{module}.json"));
    let text = fs::read_to_string(&path).expect("read a pinned specification");
    let document: Value = serde_json::from_str(&text).expect("parse a pinned specification");

    assert!(document.is_object(), "a specification is an object");

    document
}

#[test]
fn every_generated_operation_comes_from_the_specification() {
    let mut count = 0_u64;

    for (module, prefix) in PRODUCTS {
        let document = document_read(module);
        let generated = generated_read(module);

        for key in generated.keys() {
            let (variant, path) = key.split_once(' ').expect("a key is a method and a path");
            let route = path.strip_prefix(prefix).expect("a path starts with its server path");
            let item = document.get("paths").and_then(|paths| paths.get(route));
            let operation = item.and_then(|found| found.get(wire_method(variant)));

            assert!(operation.is_some(), "{key} of {module} is in no specification");

            count = count.strict_add(1);
        }
    }

    assert_eq!(count, OPERATION_COUNT_TOTAL, "every generated file was checked");
}

#[test]
fn every_member_of_every_schema_has_a_constant() {
    let mut member_count = 0_u64;

    for (module, _prefix) in PRODUCTS {
        let document = document_read(module);
        let shapes = shapes_read(module).join("\n");
        let schemas = document.pointer("/components/schemas").and_then(Value::as_object);

        let members = schemas
            .into_iter()
            .flatten()
            .filter_map(|(_name, schema)| schema.get("properties").and_then(Value::as_object));

        for wire in members.flatten().map(|(wire, _node)| wire.as_str()) {
            let literal = format!("crate::Field::new(\"{wire}\")");

            assert!(shapes.contains(&literal), "{module} has no constant for {wire}");

            member_count = member_count.strict_add(1);
        }
    }

    assert_eq!(member_count, 1_571, "the check reached every member Atlassian documents");
}

#[test]
fn every_operation_of_every_specification_has_one_generated_file() {
    let mut count = 0_u64;

    for (module, prefix) in PRODUCTS {
        let document = document_read(module);
        let generated = generated_read(module);

        for specified in specified_read(&document) {
            let key = format!("{} {prefix}{}", specified.variant, specified.path);

            assert!(generated.contains_key(&key), "{module} has no file for {key}");

            count = count.strict_add(1);
        }
    }

    assert_eq!(count, OPERATION_COUNT_TOTAL, "the specifications hold what the crate claims");
}

#[test]
fn every_operation_that_names_the_xsrf_header_sends_it() {
    let mut named_count = 0_u64;
    let mut sent_count = 0_u64;

    for (module, prefix) in PRODUCTS {
        let document = document_read(module);
        let generated = generated_read(module);

        for specified in specified_read(&document) {
            let key = format!("{} {prefix}{}", specified.variant, specified.path);
            let found = generated.get(&key).expect("a generated file");
            let declared = specified.operation.get("requestBody");
            let body = declared.map(|raw| specified.resolved(raw).to_string());
            let is_multipart = body.unwrap_or_default().contains(MULTIPART);
            let is_named = specified.operation.to_string().contains(HEADER_XSRF_NAME);
            let is_owed = is_named && !is_multipart;
            let is_sent = found.text.contains(HEADER_XSRF_SENT);

            assert_eq!(is_sent, is_owed, "{key} sends the XSRF header {is_sent}, named {is_named}");

            named_count = named_count.strict_add(u64::from(is_named));
            sent_count = sent_count.strict_add(u64::from(is_sent));
        }
    }

    assert_eq!(named_count, 9, "six watchers and three uploads name the header");
    assert_eq!(sent_count, 6, "each watcher sends it, and each upload's body carries it");
}

#[test]
fn every_parameter_and_body_the_specification_names_reaches_the_request() {
    let mut parameter_count = 0_u64;

    for (module, prefix) in PRODUCTS {
        let document = document_read(module);
        let generated = generated_read(module);

        for specified in specified_read(&document) {
            let key = format!("{} {prefix}{}", specified.variant, specified.path);
            let found = generated.get(&key).expect("a generated file");

            parameter_count = parameter_count.strict_add(parameters_check(specified, found));

            if specified.operation.get("requestBody").is_some() {
                assert!(found.text.contains("request.body_"), "{key} drops the body");
            }
        }
    }

    assert_eq!(parameter_count, 1_116, "the check reached every parameter of every product");
}

#[test]
fn every_placeholder_of_every_path_is_filled_by_its_operation() {
    let mut placeholder_count = 0_u64;

    for (module, _prefix) in PRODUCTS {
        for found in generated_read(module).values() {
            let path = path_of(&found.text);

            for piece in path.split('{').skip(1) {
                let wire = piece.split('}').next().expect("a placeholder closes");
                let quoted = format!("(\"{wire}\", self.");

                assert!(found.text.contains(&quoted), "{} leaves {wire}", found.file.display());

                placeholder_count = placeholder_count.strict_add(1);
            }
        }
    }

    assert_eq!(placeholder_count, 392, "the check reached every placeholder of every product");
}

fn generated_read(module: &str) -> BTreeMap<String, Generated> {
    assert!(!module.is_empty(), "a module is named");

    let directory = root().join("src").join(module);
    let mut generated: BTreeMap<String, Generated> = BTreeMap::new();

    for entry in fs::read_dir(&directory).expect("read a generated module") {
        let file = entry.expect("a directory entry").path();

        if !file.is_file() {
            continue;
        }

        if file.ends_with("mod.rs") {
            continue;
        }

        let text = fs::read_to_string(&file).expect("read a generated file");
        let key = format!("{} {}", method_of(&text), path_of(&text));
        let replaced = generated.insert(key.clone(), Generated { file, text });

        assert!(replaced.is_none(), "two files of {module} claim {key}");
    }

    assert!(!generated.is_empty(), "{module} holds operations");

    generated
}

fn method_of(text: &str) -> &str {
    let tail = text.split("const METHOD: crate::Method = crate::Method::").nth(1);
    let variant = tail.and_then(|found| found.split(';').next()).expect("a method");

    assert!(!variant.is_empty(), "an operation has a method");
    assert!(variant.len() <= 6, "a method is a word");

    variant
}

fn parameters_check(specified: Specified<'_>, found: &Generated) -> u64 {
    assert!(specified.item.is_object(), "a path item is an object");
    assert!(specified.operation.is_object(), "an operation is an object");

    let shared = specified.item.get("parameters").and_then(Value::as_array);
    let own = specified.operation.get("parameters").and_then(Value::as_array);
    let mut count = 0_u64;

    for declared in shared.into_iter().flatten().chain(own.into_iter().flatten()) {
        let parameter = specified.resolved(declared);
        let wire = parameter.get("name").and_then(Value::as_str).expect("a named parameter");
        let quoted = format!("\"{wire}\",");

        if wire == "Authorization" {
            continue;
        }

        assert!(found.text.contains(&quoted), "{} never sends {wire}", found.file.display());

        count = count.strict_add(1);
    }

    count
}

fn path_of(text: &str) -> String {
    let tail = text.split("const PATH: &'static str =").nth(1).expect("a path");
    let statement = tail.split(';').next().expect("a path statement");
    let pieces: Vec<&str> = statement.split('"').skip(1).step_by(2).collect();
    let path = pieces.concat();

    assert!(path.starts_with('/'), "a path is rooted at its base");
    assert!(!pieces.is_empty(), "a path holds at least one literal");

    path
}

fn root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();

    assert!(root.join("Cargo.toml").is_file(), "the manifest directory is the repository");
    assert!(root.join("spec").is_dir(), "the repository pins its specifications");

    root
}

fn shapes_read(module: &str) -> Vec<String> {
    assert!(!module.is_empty(), "a module is named");

    let directory = root().join("src").join(module).join("shape");
    let mut shapes: Vec<String> = Vec::new();

    if !directory.is_dir() {
        return shapes;
    }

    for entry in fs::read_dir(&directory).expect("read a shape module") {
        let file = entry.expect("a directory entry").path();

        shapes.push(fs::read_to_string(&file).expect("read a shape"));
    }

    assert!(!shapes.is_empty(), "a shape module holds shapes");

    shapes
}

fn specified_read(document: &Value) -> Vec<Specified<'_>> {
    assert!(document.is_object(), "a specification is an object");

    let paths = document.get("paths").and_then(Value::as_object).expect("paths");
    let mut specified: Vec<Specified<'_>> = Vec::new();

    for (path, item) in paths {
        for (method, variant) in METHODS {
            if let Some(operation) = item.get(method) {
                specified.push(Specified { document, item, operation, path, variant });
            }
        }
    }

    assert!(!specified.is_empty(), "a specification names operations");

    specified
}

fn wire_method(variant: &str) -> &'static str {
    assert!(!variant.is_empty(), "a method is named");

    let found = METHODS.iter().find(|(_wire, name)| *name == variant).expect("a known method");

    assert_eq!(found.1, variant, "the table names the variant");

    found.0
}
