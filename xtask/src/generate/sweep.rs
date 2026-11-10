use super::emit::{File, fenced};
use super::model::{Body, BodyKind, Model, Operation, Parameter, ParameterKind};
use super::name::{function_name, module_name};
use super::product::Product;

const INDENT_STATEMENT: u64 = 4;
const LINE_COLUMN_MAX: u64 = 100;
const MODULES: [&str; 3] = ["automation", "v1", "v2"];
const NAMED: &str = "the step names this operation";
const REPLY_BINARY: &str = "crate::Binary";
const SPENT: &str = "reads every value the step holds";

#[derive(Debug)]
struct Fields {
    optional: Vec<String>,
    required: Vec<String>,
}

fn body_field(body: Body) -> String {
    let multipart_optional = (BodyKind::Multipart, false);

    assert_ne!((body.kind, body.is_required), multipart_optional, "a step reads parts as required");

    let expression = match body.kind {
        BodyKind::JSONValue if body.is_required => "step.body_required()",
        BodyKind::JSONValue => "step.body()",
        BodyKind::Multipart => "step.parts_required()",
    };

    assert!(expression.starts_with("step."), "a body is read from the step");

    format!("body: {expression}")
}

fn call_lines(operation: &Operation, fields: &Fields) -> Vec<String> {
    assert!(!operation.name.is_empty(), "an operation is named");

    let has_options = !fields.optional.is_empty();
    let is_download = operation.reply == REPLY_BINARY;
    let mut lines: Vec<String> = Vec::new();

    if is_download {
        assert!(has_options, "{} downloads with options", operation.identifier);
    }

    if operation.paging.is_some() {
        assert!(has_options, "{} walks with options", operation.identifier);

        lines.push("if step.is_walk() {".to_owned());
        lines.push("return step.walk(client, &operation, options);".to_owned());
        lines.push("}".to_owned());
        lines.push(String::new());
    }

    let call = match (is_download, has_options) {
        (true, _) => "step.download(client, &operation, &options)",
        (false, true) => "step.send(client, &operation, &options)",
        (false, false) => "step.send_bare(client, &operation)",
    };

    lines.push(call.to_owned());

    assert!(lines.last().is_some_and(|line| line.starts_with("step.")), "the call is last");

    lines
}

fn declaration_line(head: &str, fields: &[String]) -> String {
    assert!(head.starts_with("let "), "a declaration binds a name");

    if fields.is_empty() {
        return format!("{head};");
    }

    let line = format!("{head} {{ {} }};", fields.join(", "));

    assert!(line.ends_with(" };"), "a literal closes its fields");

    line
}

fn fields_of(operation: &Operation) -> Fields {
    assert!(!operation.name.is_empty(), "an operation is named");

    let mut optional: Vec<(String, String)> = Vec::new();
    let mut required: Vec<(String, String)> = Vec::new();

    for parameter in &operation.parameters {
        let named = (parameter.name.clone(), parameter_field(parameter));

        if parameter.is_required {
            required.push(named);
        } else {
            optional.push(named);
        }
    }

    if let Some(body) = operation.body {
        let named = ("body".to_owned(), body_field(body));

        if body.is_required {
            required.push(named);
        } else {
            optional.push(named);
        }
    }

    optional.sort();
    required.sort();

    let input_count = operation.parameters.len().strict_add(usize::from(operation.body.is_some()));

    let fields = Fields {
        optional: optional.into_iter().map(|(_name, field)| field).collect(),
        required: required.into_iter().map(|(_name, field)| field).collect(),
    };

    assert_eq!(fields.optional.len().strict_add(fields.required.len()), input_count, "one each");

    fields
}

#[must_use]
pub(crate) fn file(product: &Product, model: &Model) -> File {
    assert!(!product.module.is_empty(), "a product names its module");
    assert!(!model.operations.is_empty(), "a product holds operations");

    let module = product.module;
    let mut operations: Vec<&Operation> = model.operations.iter().collect();

    operations.sort_by_key(|operation| function_name(&operation.name));

    let mut lines = vec![
        format!("use confluence::{module};"),
        "use confluence::{Client, Error, Transport};".to_owned(),
        String::new(),
        "use crate::step::{Run, Sent, Step};".to_owned(),
        String::new(),
        "pub(crate) trait Operations: Transport + Sized {".to_owned(),
        format!("const ALL: [(&'static str, Run<Self>); {}] = [", operations.len()),
    ];

    for operation in &operations {
        let name = function_name(&operation.name);

        lines.push(format!("(\"{}\", {name}::<Self>),", operation.identifier));
    }

    lines.push("];".to_owned());
    lines.push("}".to_owned());
    lines.push(String::new());
    lines.push("impl<Network> Operations for Network where Network: Transport {}".to_owned());

    for operation in &operations {
        lines.push(String::new());
        lines.extend(function_lines(module, operation));
    }

    assert!(lines.len() > operations.len(), "each operation holds lines");

    File { lines, path: format!("{module}.rs") }
}

fn function_lines(module: &str, operation: &Operation) -> Vec<String> {
    assert!(!operation.identifier.is_empty(), "an operation is named by its specification");
    assert!(operation.identifier.is_ascii(), "an operation id needs no escape");

    let identifier = &operation.identifier;
    let name = function_name(&operation.name);
    let fields = fields_of(operation);
    let operation_head = format!("let operation = {module}::{}", operation.name);
    let mut declarations = vec![declaration_line(&operation_head, &fields.required)];

    if !fields.optional.is_empty() {
        let options_module = module_name(&operation.name);
        let options_head = format!("let options = {module}::{options_module}::Options");

        declarations.push(declaration_line(&options_head, &fields.optional));
    }

    let mut lines = vec![
        format!("pub(crate) fn {name}<Network>(client: &Client<Network>, step: &Step<'_>)"),
        "-> Result<Sent, Error>".to_owned(),
        "where".to_owned(),
        "Network: Transport,".to_owned(),
        "{".to_owned(),
        macro_line(&format!("assert_eq!(step.identifier(), \"{identifier}\", \"{NAMED}\"")),
        String::new(),
    ];

    lines.extend(fenced(declarations, INDENT_STATEMENT));
    lines.push(String::new());
    lines.push(macro_line(&format!("assert!(step.is_spent(), \"{identifier} {SPENT}\"")));
    lines.push(String::new());
    lines.extend(call_lines(operation, &fields));
    lines.push("}".to_owned());

    assert!(lines.len() >= 11, "a function holds its header, assertions, and call");

    lines
}

fn macro_line(call_open: &str) -> String {
    assert!(call_open.contains("!("), "a macro call opens its arguments");
    assert!(!call_open.ends_with(')'), "the call is closed here");

    let line = format!("{call_open});");
    let width = u64::try_from(line.len()).unwrap_or(u64::MAX);

    if INDENT_STATEMENT.strict_add(width) <= LINE_COLUMN_MAX {
        return line;
    }

    format!("{call_open},);")
}

fn parameter_field(parameter: &Parameter) -> String {
    assert!(!parameter.wire.is_empty(), "a parameter is named");
    assert!(!parameter.wire.contains(['"', '\\']), "a wire name needs no escape");

    let accessor = match parameter.kind {
        ParameterKind::Boolean => "boolean",
        ParameterKind::Number => "number",
        ParameterKind::Numbers => "numbers",
        ParameterKind::Pairs => "pairs",
        ParameterKind::Text => "text",
        ParameterKind::Texts | ParameterKind::TextsJoined => "texts",
    };

    if parameter.is_required {
        let has_required_accessor = matches!(accessor, "number" | "text" | "texts");

        assert!(has_required_accessor, "a required {accessor} needs a step accessor");
    }

    let suffix = if parameter.is_required { "_required" } else { "" };

    format!("{}: step.{accessor}{suffix}(\"{}\")", parameter.name, parameter.wire)
}

#[must_use]
pub(crate) fn root() -> File {
    let lines: Vec<String> =
        MODULES.iter().map(|module| format!("pub(crate) mod {module};")).collect();

    assert_eq!(lines.len(), MODULES.len(), "each product holds a module");
    assert!(lines.iter().all(|line| line.ends_with(';')), "each line declares a module");

    File { lines, path: "mod.rs".to_owned() }
}
