use alloc::collections::BTreeSet;

use super::model::{
    Body,
    BodyKind,
    Location,
    Model,
    Operation,
    Paging,
    Parameter,
    ParameterKind,
    SCHEMA_SCOPE,
    SchemaField,
};
use super::name::module_name;
use super::product::Product;

const LIFETIME_OPERATION: &str = "call";
const LIFETIME_OPTIONS: &str = "options";
const INDENT_MEMBER: u64 = 4;
const INDENT_STATEMENT: u64 = 8;
const LINE_COLUMN_MAX: u64 = 100;
const PATH_CHUNK_COLUMN_MAX: u64 = 88;
const SHAPE_FROM_OPERATION: &str = "super::shape::";
const SHAPE_FROM_SHAPE: &str = "super::";

#[derive(Debug)]
struct FieldLine {
    field_type: String,
    name: String,
}

#[derive(Clone, Copy, Debug)]
struct Fields<'fields> {
    optional: &'fields [FieldLine],
    required: &'fields [FieldLine],
}

#[derive(Debug)]
pub(crate) struct File {
    pub lines: Vec<String>,
    pub path: String,
}

fn body_call(body: Body, source: &str) -> String {
    assert!(!source.is_empty(), "a body comes from a field");

    let call = match body.kind {
        BodyKind::JSONValue => format!("request.body_json({source})?;"),
        BodyKind::Multipart => format!("request.body_multipart({source})?;"),
    };

    assert!(call.ends_with(';'), "a body call is one statement");

    call
}

fn body_lines(operation: &Operation) -> Vec<String> {
    assert!(!operation.name.is_empty(), "an operation is named");

    let Some(body) = operation.body else {
        return Vec::new();
    };

    if body.is_required {
        return vec![body_call(body, "self.body")];
    }

    vec!["if let Some(body) = options.body {".to_owned(), body_call(body, "body"), "}".to_owned()]
}

fn body_type(body: Body, lifetime: &str) -> String {
    assert!(!lifetime.is_empty(), "a body borrows for a named lifetime");

    let text = match body.kind {
        BodyKind::JSONValue => format!("&'{lifetime} serde_json::Value"),
        BodyKind::Multipart => format!("&'{lifetime} [crate::Part<'{lifetime}>]"),
    };

    assert!(text.contains('\''), "a body is always borrowed");

    text
}

fn export_lines(name: &str) -> Vec<String> {
    assert!(!name.is_empty(), "an export names its type");

    let module = module_name(name);
    let line = format!("pub use {module}::{name};");

    if width_of(&line) <= LINE_COLUMN_MAX {
        return vec![line];
    }

    let lines = vec![format!("pub use {module}::"), format!("    {name};")];

    assert!(lines.iter().all(|found| width_of(found) <= LINE_COLUMN_MAX), "{name} fits");

    lines
}

pub(crate) fn fenced(lines: Vec<String>, indent: u64) -> Vec<String> {
    assert!(indent >= INDENT_MEMBER, "a fenced line sits inside a block");
    assert!(lines.iter().all(|line| !line.contains('\n')), "a fenced line is one line");

    let mut fence: Vec<String> = Vec::with_capacity(lines.len());
    let line_count = lines.len();

    for (index, line) in lines.into_iter().enumerate() {
        if indent.strict_add(width_of(&line)) <= LINE_COLUMN_MAX {
            fence.push(line);

            continue;
        }

        if !fence.last().is_none_or(String::is_empty) {
            fence.push(String::new());
        }

        fence.push(line);

        if index.strict_add(1) < line_count {
            fence.push(String::new());
        }
    }

    assert!(fence.len() >= line_count, "a fence only ever adds lines");

    fence
}

fn field_borrows(field: &FieldLine) -> bool {
    assert!(!field.field_type.is_empty(), "a field has a type");
    assert!(!field.name.is_empty(), "a field has a name");

    field.field_type.contains('\'')
}

fn field_names_are_unique(fields: &[FieldLine]) -> bool {
    let names: BTreeSet<&str> = fields.iter().map(|field| field.name.as_str()).collect();

    assert!(names.len() <= fields.len(), "a set holds at most what it was given");

    names.len() == fields.len()
}

#[must_use]
pub(crate) fn files(product: &Product, model: &Model) -> Vec<File> {
    assert!(!product.module.is_empty(), "a product names its module");
    assert!(!model.operations.is_empty(), "a product holds operations");

    let mut found: Vec<File> = Vec::new();

    for operation in &model.operations {
        found.push(File {
            lines: operation_lines(product, operation),
            path: format!("{}/{}.rs", product.module, module_name(&operation.name)),
        });
    }

    for (name, fields) in &model.schemas {
        found.push(File {
            lines: schema_lines(name, fields),
            path: format!("{}/shape/{}.rs", product.module, module_name(name)),
        });
    }

    found.push(File { lines: product_lines(model), path: format!("{}/mod.rs", product.module) });

    if !model.schemas.is_empty() {
        found.push(File {
            lines: schema_root_lines(model),
            path: format!("{}/shape/mod.rs", product.module),
        });
    }

    assert!(found.len() >= model.operations.len(), "each operation holds a file");

    found
}

fn operation_implementation(
    product: &Product,
    operation: &Operation,
    fields: Fields<'_>,
) -> Vec<String> {
    assert!(!operation.name.is_empty(), "an operation is named");
    assert!(!product.base.is_empty(), "a product names its base");

    let header = struct_header(&operation.name, fields.required);
    let options_name = if fields.optional.is_empty() { "_options" } else { "options" };
    let reply = operation.reply.replace(SCHEMA_SCOPE, SHAPE_FROM_OPERATION);

    let mut lines = vec![
        format!("impl crate::Operation for {header} {{"),
        format!("const BASE: crate::Base = crate::Base::{};", product.base),
        format!("const IS_READ: bool = {};", operation.is_read),
        format!("const METHOD: crate::Method = crate::Method::{};", operation.method),
    ];

    lines.extend(path_lines(&operation.path));

    lines = fenced(lines, INDENT_MEMBER);

    let types = vec![
        format!("type Options<'options> = {};", options_type(fields.optional)),
        format!("type Reply = {reply};"),
    ];

    lines.push(String::new());
    lines.extend(fenced(types, INDENT_MEMBER));

    lines.extend([
        String::new(),
        format!("fn request(&self, {options_name}: &Self::Options<'_>,"),
        "request: &mut crate::Request) -> Result<(), crate::Error> {".to_owned(),
        "assert!(request.is_blank(), \"a request starts blank\");".to_owned(),
        String::new(),
    ]);

    let statements = request_statements(operation);

    if !statements.is_empty() {
        lines.extend(statements);
        lines.push(String::new());
    }

    lines.extend([
        "assert!(request.is_complete(), \"every path parameter is filled\");".to_owned(),
        String::new(),
        "Ok(())".to_owned(),
        "}".to_owned(),
        "}".to_owned(),
    ]);

    lines
}

fn operation_lines(product: &Product, operation: &Operation) -> Vec<String> {
    assert!(!operation.name.is_empty(), "an operation is named");
    assert!(operation.path.starts_with('/'), "a path is rooted at its base");

    let required = required_fields(operation);
    let optional = optional_fields(operation);
    let fields = Fields { optional: &optional, required: &required };
    let mut lines = operation_struct(operation, &required);

    if !optional.is_empty() {
        lines.extend(options_struct(&optional));
    }

    lines.extend(operation_implementation(product, operation, fields));

    if let Some(paging) = &operation.paging {
        lines.extend(paged_implementation(operation, paging, &required));
    }

    lines
}

fn operation_struct(operation: &Operation, required: &[FieldLine]) -> Vec<String> {
    assert!(!operation.name.is_empty(), "an operation is named");

    let mut lines = vec!["#[derive(Clone, Copy, Debug)]".to_owned()];

    if required.is_empty() {
        lines.push(format!("pub struct {};", operation.name));
        lines.push(String::new());

        return lines;
    }

    let borrows = required.iter().any(field_borrows);
    let lifetime = if borrows { format!("<'{LIFETIME_OPERATION}>") } else { String::new() };

    lines.push(format!("pub struct {}{lifetime} {{", operation.name));

    for field in required {
        lines.push(format!("pub {}: {},", field.name, field.field_type));
    }

    lines.push("}".to_owned());
    lines.push(String::new());

    assert!(lines.len() >= required.len(), "each field holds a line");

    lines
}

fn optional_fields(operation: &Operation) -> Vec<FieldLine> {
    assert!(!operation.name.is_empty(), "an operation is named");

    let mut fields: Vec<FieldLine> = operation
        .parameters
        .iter()
        .filter(|parameter| !parameter.is_required)
        .map(|parameter| {
            let field_type = parameter_type(parameter.kind, LIFETIME_OPTIONS);

            FieldLine { field_type: format!("Option<{field_type}>"), name: parameter.name.clone() }
        })
        .collect();

    let body = operation.body.filter(|found| !found.is_required);

    if let Some(found) = body {
        let field_type = body_type(found, LIFETIME_OPTIONS);

        fields.push(FieldLine {
            field_type: format!("Option<{field_type}>"),
            name: "body".to_owned(),
        });
    }

    fields.sort_by(|left, right| left.name.cmp(&right.name));

    assert!(field_names_are_unique(&fields), "a field of {} repeats", operation.name);

    fields
}

fn options_struct(optional: &[FieldLine]) -> Vec<String> {
    assert!(!optional.is_empty(), "an options struct holds options");

    let borrows = optional.iter().any(field_borrows);
    let lifetime = if borrows { format!("<'{LIFETIME_OPTIONS}>") } else { String::new() };

    let mut lines = vec![
        "#[derive(Clone, Copy, Debug, Default)]".to_owned(),
        format!("pub struct Options{lifetime} {{"),
    ];

    for field in optional {
        lines.push(format!("pub {}: {},", field.name, field.field_type));
    }

    lines.push("}".to_owned());
    lines.push(String::new());

    assert!(lines.len() >= optional.len(), "each field holds a line");

    lines
}

fn options_type(optional: &[FieldLine]) -> String {
    if optional.is_empty() {
        return "()".to_owned();
    }

    if optional.iter().any(field_borrows) {
        return "Options<'options>".to_owned();
    }

    assert!(!optional.is_empty(), "a named options type holds options");

    "Options".to_owned()
}

fn paged_implementation(
    operation: &Operation,
    paging: &Paging,
    required: &[FieldLine],
) -> Vec<String> {
    assert!(!paging.cursor.is_empty(), "a walk names its cursor");
    assert!(!operation.name.is_empty(), "an operation is named");

    let header = struct_header(&operation.name, required);
    let cursor = &paging.cursor;
    let items = &paging.items;
    let next = &paging.next;

    vec![
        String::new(),
        format!("impl crate::Paged for {header} {{"),
        format!("const PAGING: crate::Paging = crate::Paging {{ cursor: \"{cursor}\","),
        format!("items: \"{items}\", next: {next} }};"),
        String::new(),
        format!("type Item = {};", paging.item.replace(SCHEMA_SCOPE, SHAPE_FROM_OPERATION)),
        "}".to_owned(),
    ]
}

fn parameter_call(parameter: &Parameter) -> String {
    assert!(!parameter.wire.is_empty(), "a parameter is named");
    assert!(!parameter.name.is_empty(), "a parameter is named");

    let wire = &parameter.wire;
    let name = &parameter.name;

    match parameter.location {
        Location::Header => format!("request.header_text(\"{wire}\", options.{name})?;"),
        Location::Path => {
            if parameter.kind == ParameterKind::Number {
                return format!("request.path_number(\"{wire}\", self.{name})?;");
            }

            format!("request.path_text(\"{wire}\", self.{name})?;")
        }
        Location::Query => query_call(parameter),
    }
}

fn parameter_type(kind: ParameterKind, lifetime: &str) -> String {
    assert!(!lifetime.is_empty(), "a parameter borrows for a named lifetime");

    let text = match kind {
        ParameterKind::Boolean => "bool".to_owned(),
        ParameterKind::Number => "i64".to_owned(),
        ParameterKind::Numbers => format!("&'{lifetime} [i64]"),
        ParameterKind::Pairs => format!("&'{lifetime} [(&'{lifetime} str, &'{lifetime} str)]"),
        ParameterKind::Text => format!("&'{lifetime} str"),
        ParameterKind::Texts | ParameterKind::TextsJoined => {
            format!("&'{lifetime} [&'{lifetime} str]")
        }
    };

    assert!(!text.is_empty(), "a parameter has a type");

    text
}

fn path_lines(path: &str) -> Vec<String> {
    assert!(path.starts_with('/'), "a path is rooted at its base");

    if width_of(path) <= PATH_CHUNK_COLUMN_MAX {
        return vec![format!("const PATH: &'static str = \"{path}\";")];
    }

    let mut lines = vec![String::new(), "const PATH: &'static str = concat!(".to_owned()];

    for chunk in path_split(path) {
        lines.push(format!("\"{chunk}\","));
    }

    lines.push(");".to_owned());

    assert!(lines.len() >= 5, "a split path spans at least two chunks");

    lines
}

fn path_split(path: &str) -> Vec<String> {
    assert!(path.starts_with('/'), "a path is rooted at its base");

    let mut chunks: Vec<String> = Vec::new();
    let mut chunk = String::new();

    for segment in path.split_inclusive('/') {
        if width_of(&chunk).strict_add(width_of(segment)) > PATH_CHUNK_COLUMN_MAX {
            chunks.push(chunk.clone());
            chunk.clear();
        }

        chunk.push_str(segment);
    }

    chunks.push(chunk);

    assert_eq!(chunks.concat(), path, "a split path joins back into itself");
    assert!(chunks.iter().all(|found| !found.is_empty()), "a chunk holds at least one segment");

    chunks
}

fn product_lines(model: &Model) -> Vec<String> {
    assert!(!model.operations.is_empty(), "a product holds operations");

    let mut lines: Vec<String> = Vec::new();

    for operation in &model.operations {
        lines.push(format!("pub mod {};", module_name(&operation.name)));
    }

    if !model.schemas.is_empty() {
        lines.push("pub mod shape;".to_owned());
    }

    lines.sort();
    lines.push(String::new());

    for operation in &model.operations {
        lines.extend(export_lines(&operation.name));
    }

    assert!(lines.len() >= model.operations.len(), "each operation holds a line");

    lines
}

fn query_call(parameter: &Parameter) -> String {
    assert_eq!(parameter.location, Location::Query, "only a query term reaches a query call");

    let wire = &parameter.wire;
    let name = &parameter.name;

    let (call, refusal) = match parameter.kind {
        ParameterKind::Boolean => ("query_boolean", ""),
        ParameterKind::Number => ("query_number", ""),
        ParameterKind::Numbers => ("query_numbers", ""),
        ParameterKind::Pairs => ("query_pairs", "?"),
        ParameterKind::Text => ("query_text", ""),
        ParameterKind::Texts => ("query_texts", ""),
        ParameterKind::TextsJoined => ("query_texts_joined", "?"),
    };

    if parameter.is_required {
        return format!("request.{call}(\"{wire}\", Some(self.{name})){refusal};");
    }

    format!("request.{call}(\"{wire}\", options.{name}){refusal};")
}

fn request_statements(operation: &Operation) -> Vec<String> {
    assert!(!operation.name.is_empty(), "an operation is named");

    let mut lines: Vec<String> = operation.parameters.iter().map(parameter_call).collect();

    if let Some((name, value)) = operation.header {
        lines.push(format!("request.header(\"{name}\", \"{value}\");"));
    }

    let body = body_lines(operation);

    if body.len() == 1 {
        lines.extend(body);

        return fenced(lines, INDENT_STATEMENT);
    }

    let mut statements = fenced(lines, INDENT_STATEMENT);

    if !body.is_empty() {
        if !statements.is_empty() {
            statements.push(String::new());
        }

        statements.extend(body);
    }

    assert!(statements.len() >= operation.parameters.len(), "each parameter holds a statement");

    statements
}

fn required_fields(operation: &Operation) -> Vec<FieldLine> {
    assert!(!operation.name.is_empty(), "an operation is named");

    let mut fields: Vec<FieldLine> = operation
        .parameters
        .iter()
        .filter(|parameter| parameter.is_required)
        .map(|parameter| FieldLine {
            field_type: parameter_type(parameter.kind, LIFETIME_OPERATION),
            name: parameter.name.clone(),
        })
        .collect();

    let body = operation.body.filter(|found| found.is_required);

    if let Some(found) = body {
        fields.push(FieldLine {
            field_type: body_type(found, LIFETIME_OPERATION),
            name: "body".to_owned(),
        });
    }

    fields.sort_by(|left, right| left.name.cmp(&right.name));

    assert!(field_names_are_unique(&fields), "a field of {} repeats", operation.name);

    fields
}

fn schema_lines(name: &str, fields: &[SchemaField]) -> Vec<String> {
    assert!(!name.is_empty(), "a schema is named");
    assert!(!fields.is_empty(), "a schema holds members");

    let mut lines = vec![
        "#[derive(Clone, Copy, Debug)]".to_owned(),
        format!("pub struct {name};"),
        String::new(),
        format!("impl {name} {{"),
    ];

    let own = format!("{SCHEMA_SCOPE}{name}>");

    let members: Vec<String> = fields
        .iter()
        .map(|field| {
            let kind = field.kind.replace(&own, "Self>").replace(SCHEMA_SCOPE, SHAPE_FROM_SHAPE);

            format!(
                "pub const {}: crate::Field<Self, {kind}> = crate::Field::new(\"{}\");",
                field.constant,
                field.wire.replace('"', "\\\""),
            )
        })
        .collect();

    lines.extend(fenced(members, INDENT_MEMBER));
    lines.push("}".to_owned());

    assert!(lines.len() > fields.len(), "each member holds a line");

    lines
}

fn schema_root_lines(model: &Model) -> Vec<String> {
    assert!(!model.schemas.is_empty(), "a shape module holds schemas");

    let mut lines: Vec<String> = Vec::new();

    for name in model.schemas.keys() {
        lines.push(format!("mod {};", module_name(name)));
    }

    lines.sort();
    lines.push(String::new());

    for name in model.schemas.keys() {
        lines.extend(export_lines(name));
    }

    assert!(lines.len() > model.schemas.len(), "each schema holds a line");

    lines
}

fn struct_header(name: &str, required: &[FieldLine]) -> String {
    assert!(!name.is_empty(), "a struct is named");

    if required.iter().any(field_borrows) {
        return format!("{name}<'_>");
    }

    assert!(!name.contains('<'), "an owned operation names no lifetime");

    name.to_owned()
}

fn width_of(text: &str) -> u64 {
    let width = u64::try_from(text.len()).unwrap_or(u64::MAX);

    assert!(width < u64::MAX, "a width fits its count");
    assert_eq!(width == 0, text.is_empty(), "a width is zero exactly when the text is empty");

    width
}

#[cfg(test)]
mod tests {
    use super::{export_lines, fenced, path_split};

    #[test]
    fn a_line_rustfmt_will_wrap_is_fenced_from_its_neighbours() {
        let long = "x".repeat(99);
        let lines = vec!["a".to_owned(), long.clone(), "b".to_owned()];

        assert_eq!(
            fenced(lines, 4),
            ["a".to_owned(), String::new(), long.clone(), String::new(), "b".to_owned()],
            "a blank line on each side",
        );

        assert_eq!(fenced(vec![long.clone()], 4), [long], "a lone line needs no fence");
        assert_eq!(fenced(vec!["a".to_owned()], 4), ["a"], "a short line stays as it is");
    }

    #[test]
    fn a_long_export_wraps_where_rust_allows_and_fits_the_line() {
        let name = "LongName".repeat(9);
        let lines = export_lines(&name);

        assert_eq!(lines.len(), 2, "an export over the limit takes two lines");
        assert!(lines.iter().all(|line| line.len() <= 100), "each line fits");
        assert_eq!(export_lines("Get"), ["pub use get::Get;"], "a short export takes one line");
    }

    #[test]
    fn a_long_path_splits_on_segments_and_joins_back_into_itself() {
        let path = format!("/{}", ["segment"; 30].join("/"));
        let chunks = path_split(&path);

        assert!(chunks.len() >= 2, "the path is long enough to split");
        assert_eq!(chunks.concat(), path, "nothing was lost");
        assert!(chunks.iter().all(|chunk| chunk.len() <= 88), "each chunk fits");
    }
}
