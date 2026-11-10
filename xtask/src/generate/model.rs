use alloc::collections::{BTreeMap, BTreeSet};
use core::{iter, ptr};

use serde_json::{Map, Value};

use super::name::{constant_name, field_name, type_name};
use super::overrides::{
    BINARY_REPLIES,
    FIELD_KINDS,
    INLINE_NAMES,
    MEMBER_NAMES,
    PARAMETER_NAMES,
    PARAMETER_TEXTS,
    READS_BY_POST,
};
use super::product::Product;

const ALL_OF_MEMBER_COUNT_MAX: u32 = 64;
const CONTENT_JSON: &str = "application/json";
const CONTENT_MULTIPART: &str = "multipart/form-data";
const CONTENT_WILDCARD: &str = "*/*";
const CURSORS: [&str; 2] = ["cursor", "start"];
const HEADER_XSRF: (&str, &str) = ("X-Atlassian-Token", "no-check");
const ITEM_FIELDS: [&str; 2] = ["data", "results"];
const KIND_ANY: &str = "crate::Any";
const KIND_BINARY: &str = "crate::Binary";
const KIND_DEPTH_MAX: u32 = 32;
const KIND_NOTHING: &str = "crate::Nothing";
const LINK_CONTAINERS: [&str; 2] = ["_links", "links"];

const METHODS: [(&str, &str); 5] =
    [("delete", "Delete"), ("get", "Get"), ("patch", "Patch"), ("post", "Post"), ("put", "Put")];

const PENDING_ROUND_COUNT_MAX: u32 = 1_000_000;
const REFERENCE_PREFIX: &str = "#/components/";
const SCHEMA_PREFIX: &str = "#/components/schemas/";
pub(crate) const SCHEMA_SCOPE: &str = "SCHEMA::";
const STATUS_NO_CONTENT: &str = "204";
const STYLE_DEEP_OBJECT: &str = "deepObject";
const TOKEN_FIELDS: [&str; 1] = ["cursor"];

#[derive(Clone, Copy, Debug)]
pub(crate) struct Body {
    pub is_required: bool,
    pub kind: BodyKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BodyKind {
    JSONValue,
    Multipart,
}

#[derive(Debug)]
struct ItemField<'document> {
    items: String,
    schema: Option<&'document Value>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Location {
    Header,
    Path,
    Query,
}

#[derive(Debug)]
pub(crate) struct Model {
    pub identifiers: BTreeSet<String>,
    pub operations: Vec<Operation>,
    pub schemas: BTreeMap<String, Vec<SchemaField>>,
}

#[derive(Debug)]
pub(crate) struct Operation {
    pub body: Option<Body>,
    pub header: Option<(&'static str, &'static str)>,
    pub identifier: String,
    pub is_read: bool,
    pub method: &'static str,
    pub name: String,
    pub paging: Option<Paging>,
    pub parameters: Vec<Parameter>,
    pub path: String,
    pub reply: String,
}

#[derive(Debug)]
pub(crate) struct Paging {
    pub cursor: String,
    pub item: String,
    pub items: String,
    pub next: String,
}

#[derive(Debug)]
pub(crate) struct Parameter {
    pub is_required: bool,
    pub kind: ParameterKind,
    pub location: Location,
    pub name: String,
    pub wire: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ParameterKind {
    Boolean,
    Number,
    Numbers,
    Pairs,
    Text,
    Texts,
    TextsJoined,
}

#[derive(Debug)]
struct PageSource<'document> {
    field: ItemField<'document>,
    next: String,
}

#[derive(Debug)]
struct Pending<'document> {
    name: String,
    nodes: Vec<&'document Value>,
}

#[derive(Debug)]
struct Registry<'document> {
    document: &'document Value,
    names: BTreeMap<String, Vec<&'document Value>>,
    pending: Vec<Pending<'document>>,
    schemas: BTreeMap<String, Vec<SchemaField>>,
}

#[derive(Debug)]
pub(crate) struct SchemaField {
    pub constant: String,
    pub kind: String,
    pub wire: String,
}

#[derive(Clone, Copy, Debug)]
struct Spot<'document> {
    item: &'document Value,
    method: &'static str,
    operation: &'document Value,
    path: &'document str,
    variant: &'static str,
}

#[derive(Debug)]
enum Step<'document> {
    Follow(&'document Value),
    Leaf(String),
    Wrap(&'static str, &'document Value),
}

impl<'document> Registry<'document> {
    fn drain(&mut self) {
        assert!(self.document.is_object(), "a registry reads a document");
        assert!(self.names.len() >= self.schemas.len(), "every schema was registered first");

        for _round_index in 0..PENDING_ROUND_COUNT_MAX {
            let Some(pending) = self.pending.pop() else {
                break;
            };

            let fields = self.fields_read(&pending);
            let replaced = self.schemas.insert(pending.name, fields);

            assert!(replaced.is_none(), "a schema is emitted once");
        }

        assert!(self.pending.is_empty(), "every schema was read within the budget");
    }

    fn field_kind(&self, definitions: &[&'document Value], declared: String) -> String {
        assert!(!definitions.is_empty(), "a member is defined at least once");
        assert!(declared.starts_with("crate::"), "a declared kind is a runtime type");

        let fixed = FIELD_KINDS.iter().find(|(schema, wire, _kind)| {
            let pointer = format!("/components/schemas/{schema}/properties/{wire}");
            let held = self.document.pointer(&pointer);

            held.is_some_and(|found| definitions.iter().any(|node| ptr::eq(found, *node)))
        });

        fixed.map_or(declared, |(_schema, _wire, kind)| (*kind).to_owned())
    }

    fn fields_read(&mut self, pending: &Pending<'document>) -> Vec<SchemaField> {
        assert!(!pending.name.is_empty(), "a schema is named");

        let properties = self.properties_all(&pending.nodes);
        let mut by_constant: BTreeMap<String, SchemaField> = BTreeMap::new();

        for (wire, definitions) in properties {
            let declared = self.kind_of(&definitions, &inline_name(pending, wire));
            let kind = self.field_kind(&definitions, declared);
            let constant = member_constant(pending, wire);
            let field = SchemaField { constant: constant.clone(), kind, wire: wire.to_owned() };
            let replaced = by_constant.insert(constant, field);

            assert!(replaced.is_none(), "two members of {} share a name", pending.name);
        }

        by_constant.into_values().collect()
    }

    fn kind(&mut self, node: &'document Value, inline_name: &str) -> String {
        assert!(!inline_name.is_empty(), "an inline record is named for where it sits");

        let mut wrappers: Vec<&'static str> = Vec::new();
        let mut current = node;
        let mut text = KIND_ANY.to_owned();

        assert!(current.is_object() || current.is_boolean(), "a schema is an object or a boolean");

        for _depth_index in 0..KIND_DEPTH_MAX {
            let step = self.step(current, inline_name);

            if let Step::Leaf(found) = step {
                text = found;

                break;
            }

            if let Step::Wrap(wrapper, _inner) = step {
                wrappers.push(wrapper);
            }

            current = step_inner(&step, current);
        }

        for wrapper in wrappers.iter().rev() {
            text = format!("crate::{wrapper}<{text}>");
        }

        assert!(text.starts_with("crate::"), "a kind is a runtime type");
        assert!(wrappers.len() <= 32, "a kind peels within its budget");

        text
    }

    fn kind_of(&mut self, definitions: &[&'document Value], inline_name: &str) -> String {
        assert!(!definitions.is_empty(), "a member is defined at least once");
        assert!(!inline_name.is_empty(), "an inline record is named for where it sits");

        let mut distinct: Vec<&'document Value> = Vec::new();

        for definition in definitions {
            if !distinct.contains(definition) {
                distinct.push(definition);
            }
        }

        if let [only] = distinct.as_slice() {
            return self.kind(only, inline_name);
        }

        let records: Vec<&'document Value> =
            distinct.iter().filter_map(|node| self.record_target(node)).collect();

        if records.is_empty() {
            return self.kind_shared(&distinct, inline_name);
        }

        assert_eq!(records.len(), distinct.len(), "{inline_name} merges records only with records");

        self.record_register(inline_name.to_owned(), records);

        format!("crate::Object<{SCHEMA_SCOPE}{inline_name}>")
    }

    fn kind_shared(&mut self, definitions: &[&'document Value], inline_name: &str) -> String {
        assert!(definitions.len() >= 2, "a shared kind is read from several definitions");
        assert!(!inline_name.is_empty(), "an inline record is named for where it sits");

        let kinds: BTreeSet<String> =
            definitions.iter().map(|node| self.kind(node, inline_name)).collect();

        assert_eq!(kinds.len(), 1, "the definitions of {inline_name} disagree on a kind");

        kinds.into_iter().next().unwrap_or_else(|| KIND_ANY.to_owned())
    }

    fn new(document: &'document Value) -> Self {
        assert!(document.is_object(), "a specification is an object");

        let mut registry = Self {
            document,
            names: BTreeMap::new(),
            pending: Vec::new(),
            schemas: BTreeMap::new(),
        };

        let schemas = document.pointer("/components/schemas").and_then(Value::as_object);

        for (key, schema) in schemas.into_iter().flatten() {
            if record_is(schema) {
                registry.record_register(type_name(key), vec![schema]);
            }
        }

        assert_eq!(registry.names.len(), registry.pending.len(), "each schema is registered once");

        registry
    }

    fn properties_all(
        &self,
        nodes: &[&'document Value],
    ) -> BTreeMap<&'document str, Vec<&'document Value>> {
        assert!(self.document.is_object(), "a registry reads a document");
        assert!(!nodes.is_empty(), "a record has a definition");

        let mut found: BTreeMap<&'document str, Vec<&'document Value>> = BTreeMap::new();
        let mut stack: Vec<&'document Value> = nodes.to_vec();

        for _member_index in 0..ALL_OF_MEMBER_COUNT_MAX {
            let Some(current) = stack.pop() else {
                break;
            };

            let resolved = self.resolved(current);
            let properties = resolved.get("properties").and_then(Value::as_object);

            for (wire, property) in properties.into_iter().flatten() {
                found.entry(wire.as_str()).or_default().push(property);
            }

            let members = resolved.get("allOf").and_then(Value::as_array);

            stack.extend(members.into_iter().flatten());
        }

        assert!(stack.is_empty(), "every allOf member was read within the budget");

        found
    }

    fn properties_read(&self, node: &'document Value) -> Vec<(&'document str, &'document Value)> {
        assert!(self.document.is_object(), "a registry reads a document");
        assert!(node.is_object() || node.is_boolean(), "a schema is an object or a boolean");

        let found = self.properties_all(&[node]);

        found
            .into_iter()
            .filter_map(|(wire, definitions)| Some((wire, *definitions.last()?)))
            .collect()
    }

    fn record_register(&mut self, name: String, nodes: Vec<&'document Value>) {
        assert!(!name.is_empty(), "a schema is named");
        assert!(nodes.iter().all(|node| record_is(node)), "only a record is registered");

        if let Some(known) = self.names.get(&name) {
            assert_eq!(*known, nodes, "two different schemas are both named {name}");

            return;
        }

        let _absent = self.names.insert(name.clone(), nodes.clone());

        self.pending.push(Pending { name, nodes });
    }

    fn record_target(&self, node: &'document Value) -> Option<&'document Value> {
        assert!(node.is_object() || node.is_boolean(), "a definition is a schema");
        assert!(self.document.is_object(), "a registry reads a document");

        let mut current = node;

        for _depth_index in 0..KIND_DEPTH_MAX {
            let resolved = self.resolved(current);

            let Some(only) = all_of_single(resolved) else {
                return record_is(resolved).then_some(resolved);
            };

            current = only;
        }

        None
    }

    fn resolved(&self, node: &'document Value) -> &'document Value {
        assert!(self.document.is_object(), "a registry reads a document");

        let Some(reference) = node.get("$ref").and_then(Value::as_str) else {
            return node;
        };

        assert!(reference.starts_with(REFERENCE_PREFIX), "a reference names a component");

        self.document.pointer(reference.trim_start_matches('#')).unwrap_or(node)
    }

    fn step(&mut self, node: &'document Value, inline_name: &str) -> Step<'document> {
        assert!(!inline_name.is_empty(), "an inline record is named for where it sits");

        if let Some(reference) = node.get("$ref").and_then(Value::as_str) {
            return self.step_of_reference(reference, self.resolved(node));
        }

        if let Some(only) = all_of_single(node) {
            return Step::Follow(only);
        }

        if record_is(node) {
            self.record_register(inline_name.to_owned(), vec![node]);

            return Step::Leaf(format!("crate::Object<{SCHEMA_SCOPE}{inline_name}>"));
        }

        step_of_type(node)
    }

    fn step_of_reference(
        &mut self,
        reference: &str,
        resolved: &'document Value,
    ) -> Step<'document> {
        assert!(reference.starts_with(SCHEMA_PREFIX), "a reference names a schema");

        if !record_is(resolved) {
            return Step::Follow(resolved);
        }

        let key = reference.trim_start_matches(SCHEMA_PREFIX);
        let name = type_name(key);

        if key.contains('/') {
            self.record_register(name.clone(), vec![resolved]);
        }

        Step::Leaf(format!("crate::Object<{SCHEMA_SCOPE}{name}>"))
    }
}

impl<'document> Spot<'document> {
    fn identifier(&self) -> &'document str {
        assert!(!self.path.is_empty(), "an operation has a path");
        assert!(self.operation.is_object(), "an operation is an object");

        self.operation.get("operationId").and_then(Value::as_str).unwrap_or_default()
    }

    fn is_read(&self) -> bool {
        assert!(!self.method.is_empty(), "an operation has a method");
        assert!(!self.path.is_empty(), "an operation has a path");

        if self.method == "get" {
            return true;
        }

        READS_BY_POST.contains(&self.identifier())
    }

    fn parameters(&self) -> impl Iterator<Item = &'document Value> {
        assert!(self.item.is_object(), "a path item is an object");
        assert!(self.operation.is_object(), "an operation is an object");

        let shared = self.item.get("parameters").and_then(Value::as_array);
        let own = self.operation.get("parameters").and_then(Value::as_array);

        shared.into_iter().flatten().chain(own.into_iter().flatten())
    }
}

fn all_of_single(node: &Value) -> Option<&Value> {
    let members = node.get("allOf").and_then(Value::as_array)?;

    let [only] = members.as_slice() else {
        return None;
    };

    assert!(only.is_object(), "an allOf member is a schema");
    assert!(!members.is_empty(), "a single member comes from a filled list");

    Some(only)
}

fn body_read<'document>(
    registry: &Registry<'document>,
    operation: &'document Value,
) -> Option<Body> {
    assert!(operation.is_object(), "an operation is an object");

    let request_body = registry.resolved(operation.get("requestBody")?);
    let content = request_body.get("content").and_then(Value::as_object)?;
    let is_required = request_body.get("required").and_then(Value::as_bool).unwrap_or_default();

    assert!(!content.is_empty(), "a request body names its content");

    if content.contains_key(CONTENT_MULTIPART) {
        return Some(Body { is_required, kind: BodyKind::Multipart });
    }

    let is_json = content.iter().any(|(media, node)| {
        let schema = node.get("schema").map(|found| registry.resolved(found));

        media_is_json(media, schema)
    });

    assert!(is_json, "a body of {:?} needs a call the emitter lacks", content.keys());

    Some(Body { is_required, kind: BodyKind::JSONValue })
}

fn cursor_find(names: &[&str]) -> Option<String> {
    assert!(!CURSORS.is_empty(), "a walk has parameters to look for");

    let found = CURSORS.iter().find(|cursor| names.contains(cursor))?;

    assert!(!found.is_empty(), "a cursor is named");
    assert!(names.contains(found), "a cursor is one of the operation's parameters");

    Some((*found).to_owned())
}

fn description_names_xsrf(node: &Value) -> bool {
    assert!(node.is_object(), "a described node is an object");

    let description = node.get("description").and_then(Value::as_str);

    description.unwrap_or_default().contains(HEADER_XSRF.0)
}

fn header_is_xsrf(parameter: &Value) -> bool {
    assert!(parameter.is_object(), "a parameter is an object");

    let place = parameter.get("in").and_then(Value::as_str).unwrap_or_default();
    let wire = parameter.get("name").and_then(Value::as_str).unwrap_or_default();

    assert!(!wire.is_empty(), "a parameter is named");

    place == "header" && wire.eq_ignore_ascii_case(HEADER_XSRF.0)
}

fn header_read<'document>(
    registry: &Registry<'document>,
    spot: Spot<'document>,
) -> Option<(&'static str, &'static str)> {
    assert!(spot.operation.is_object(), "an operation is an object");
    assert!(registry.document.is_object(), "a registry reads a document");

    if body_read(registry, spot.operation).is_some_and(|found| found.kind == BodyKind::Multipart) {
        return None;
    }

    let responses = spot.operation.get("responses").and_then(Value::as_object);
    let answers = responses.into_iter().flat_map(Map::values).map(|raw| registry.resolved(raw));
    let is_described = iter::once(spot.operation).chain(answers).any(description_names_xsrf);
    let is_declared = spot.parameters().any(|raw| header_is_xsrf(registry.resolved(raw)));

    (is_described || is_declared).then_some(HEADER_XSRF)
}

fn identifiers_read(paths: &Map<String, Value>) -> BTreeSet<String> {
    assert!(!paths.is_empty(), "a specification names paths");

    let mut identifiers: BTreeSet<String> = BTreeSet::new();

    for (method, _variant) in METHODS {
        let operations = paths.values().filter_map(|item| item.get(method));

        identifiers.extend(
            operations
                .filter_map(|operation| operation.get("operationId"))
                .filter_map(Value::as_str)
                .map(str::to_owned),
        );
    }

    assert!(!identifiers.is_empty(), "a specification names operations");

    identifiers
}

fn inline_name(pending: &Pending<'_>, wire: &str) -> String {
    assert!(!pending.name.is_empty(), "an inline record sits inside a named record");
    assert!(!wire.is_empty(), "an inline record sits under a named member");

    let name = format!("{}{}", pending.name, type_name(wire));
    let found = INLINE_NAMES.iter().find(|(full, _short)| *full == name);

    found.map_or(name, |(_full, short)| (*short).to_owned())
}

fn item_field_find<'document>(
    registry: &Registry<'document>,
    properties: &[(&'document str, &'document Value)],
) -> Option<ItemField<'document>> {
    assert!(registry.document.is_object(), "a registry reads a document");

    for field in ITEM_FIELDS {
        let found = properties.iter().find(|(wire, _node)| *wire == field);

        if let Some((wire, node)) = found {
            let resolved = registry.resolved(node);

            if resolved.get("type").and_then(Value::as_str) == Some("array") {
                return Some(ItemField {
                    items: format!("/{wire}"),
                    schema: resolved.get("items"),
                });
            }
        }
    }

    None
}

const fn location_rank(location: Location) -> u32 {
    match location {
        Location::Header => 0,
        Location::Path => 1,
        Location::Query => 2,
    }
}

fn media_is_json(media: &str, schema: Option<&Value>) -> bool {
    assert!(!media.is_empty(), "a media type is named");
    assert!(schema.is_none_or(|found| found.is_object() || found.is_boolean()), "a schema");

    let essence = media.split(';').next().unwrap_or_default().trim();

    assert!(!essence.is_empty(), "a media type has an essence before its parameters");

    if essence.eq_ignore_ascii_case(CONTENT_JSON) {
        return true;
    }

    essence == CONTENT_WILDCARD && schema.is_some_and(record_is)
}

fn member_constant(pending: &Pending<'_>, wire: &str) -> String {
    let schema = pending.name.as_str();

    assert!(!schema.is_empty(), "a member belongs to a schema");
    assert!(!wire.is_empty(), "a member is named");

    let fixed =
        MEMBER_NAMES.iter().find(|(name, member, _constant)| *name == schema && *member == wire);

    fixed.map_or_else(|| constant_name(wire), |&(_name, _member, constant)| constant.to_owned())
}

pub(crate) fn model_build(product: &Product, document: &Value) -> Result<Model, String> {
    assert!(document.is_object(), "a specification is an object");
    assert!(!product.module.is_empty(), "a product names its module");

    let mut registry = Registry::new(document);
    let mut operations: Vec<Operation> = Vec::new();

    let paths = document
        .get("paths")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{} names no paths", product.module))?;

    for (path, item) in paths {
        operations.extend(operations_read(&mut registry, product, (path, item))?);
    }

    registry.drain();

    operations.sort_by(|left, right| left.name.cmp(&right.name));

    let names: BTreeSet<&String> = operations.iter().map(|found| &found.name).collect();

    if names.len() != operations.len() {
        return Err(format!("two operations of {} share a name", product.module));
    }

    assert!(!operations.is_empty(), "a product holds operations");

    Ok(Model { identifiers: identifiers_read(paths), operations, schemas: registry.schemas })
}

fn name_read(spot: Spot<'_>) -> Result<String, String> {
    assert!(!spot.path.is_empty(), "an operation has a path");

    let identifier = spot.identifier();

    if identifier.is_empty() {
        return Err(format!("{} {} names no operationId", spot.method, spot.path));
    }

    assert!(!identifier.is_empty(), "an operation is named by its specification");

    Ok(type_name(identifier))
}

fn operation_read<'document>(
    registry: &mut Registry<'document>,
    product: &Product,
    spot: Spot<'document>,
) -> Result<Operation, String> {
    assert!(!spot.path.is_empty(), "an operation has a path");
    assert!(spot.operation.is_object(), "an operation is an object");

    let name = name_read(spot)?;
    let mut parameters = parameters_read(registry, spot);

    let path = format!("{}{}", product.path_prefix, spot.path);

    placeholders_declare(&path, &mut parameters);
    placeholders_check(&path, &parameters)?;

    let is_binary = reply_is_binary(registry, spot);
    let reply_node = if is_binary { None } else { reply_schema(registry, spot.operation) };

    let reply = if is_binary {
        KIND_BINARY.to_owned()
    } else {
        reply_kind(registry, spot, reply_node, &name)
    };

    let paging = paging_read(registry, spot, &parameters, reply_node, &name);
    let body = body_read(registry, spot.operation);

    parameters.sort_by(|left, right| left.name.cmp(&right.name));

    Ok(Operation {
        body,
        header: header_read(registry, spot),
        identifier: spot.identifier().to_owned(),
        is_read: spot.is_read(),
        method: spot.variant,
        name,
        paging,
        parameters,
        path,
        reply,
    })
}

fn operations_read<'document>(
    registry: &mut Registry<'document>,
    product: &Product,
    (path, item): (&'document str, &'document Value),
) -> Result<Vec<Operation>, String> {
    assert!(!path.is_empty(), "a path is named");
    assert!(item.is_object(), "a path item is an object");

    let mut operations: Vec<Operation> = Vec::new();

    for (method, variant) in METHODS {
        if let Some(operation) = item.get(method) {
            let spot = Spot { item, method, operation, path, variant };

            operations.push(operation_read(registry, product, spot)?);
        }
    }

    assert!(operations.len() <= METHODS.len(), "a path holds one operation per method");

    Ok(operations)
}

fn page_source_link<'document>(
    registry: &Registry<'document>,
    properties: &[(&'document str, &'document Value)],
) -> Option<PageSource<'document>> {
    assert!(registry.document.is_object(), "a registry reads a document");

    let (container, node) =
        properties.iter().find(|(wire, _node)| LINK_CONTAINERS.contains(wire))?;

    let links = registry.resolved(node);
    let inner = registry.properties_read(links);
    let is_declared = inner.iter().any(|(wire, _node)| *wire == "next");
    let additional = links.get("additionalProperties");
    let is_open = additional.is_some_and(|found| *found != Value::Bool(false));
    let is_linked = is_declared || is_open;

    if !is_linked {
        return None;
    }

    let field = item_field_find(registry, properties)?;

    assert!(LINK_CONTAINERS.contains(container), "a link sits in a link container");

    Some(PageSource { field, next: format!("crate::Next::Link(\"/{container}/next\")") })
}

fn page_source_token<'document>(
    registry: &Registry<'document>,
    properties: &[(&'document str, &'document Value)],
) -> Option<PageSource<'document>> {
    assert!(registry.document.is_object(), "a registry reads a document");

    let token = token_field_find(properties)?;
    let field = item_field_find(registry, properties)?;

    assert!(TOKEN_FIELDS.contains(&token.as_str()), "a token is one the table spells");

    Some(PageSource { field, next: format!("crate::Next::Token(\"/{token}\")") })
}

fn paging_read<'document>(
    registry: &mut Registry<'document>,
    spot: Spot<'document>,
    parameters: &[Parameter],
    reply: Option<&'document Value>,
    name: &str,
) -> Option<Paging> {
    assert!(!name.is_empty(), "an operation is named");

    if spot.method != "get" {
        return None;
    }

    let names: Vec<&str> = parameters.iter().map(|found| found.wire.as_str()).collect();
    let cursor = cursor_find(&names)?;
    let resolved = registry.resolved(reply?);
    let properties = registry.properties_read(resolved);

    let source = page_source_link(registry, &properties)
        .or_else(|| page_source_token(registry, &properties))?;

    let schema = source.field.schema;
    let item_name = format!("{name}Item");
    let item = schema.map_or_else(|| KIND_ANY.to_owned(), |found| registry.kind(found, &item_name));

    assert!(source.field.items.starts_with('/'), "an array is named by a pointer");

    Some(Paging { cursor, item, items: source.field.items, next: source.next })
}

fn parameter_kind(registry: &Registry<'_>, parameter: &Value) -> ParameterKind {
    assert!(registry.document.is_object(), "a registry reads a document");

    let wire = parameter.get("name").and_then(Value::as_str).unwrap_or_default();
    let schema = registry.resolved(parameter.get("schema").unwrap_or(&Value::Null));
    let shape = schema.get("type").and_then(Value::as_str);

    if parameter.get("style").and_then(Value::as_str) == Some(STYLE_DEEP_OBJECT) {
        assert_eq!(shape, Some("object"), "the deep object {wire} holds named members");

        return ParameterKind::Pairs;
    }

    assert_ne!(shape, Some("object"), "the object {wire} needs a style the emitter lacks");

    match shape {
        Some("array") => {
            let is_exploded = parameter.get("explode").and_then(Value::as_bool).unwrap_or(true);
            let items = schema.get("items").map(|found| registry.resolved(found));
            let item_type = items.and_then(|found| found.get("type")).and_then(Value::as_str);

            if item_type == Some("integer") {
                assert!(is_exploded, "{wire} needs a list of numbers the emitter lacks");

                return ParameterKind::Numbers;
            }

            if is_exploded { ParameterKind::Texts } else { ParameterKind::TextsJoined }
        }
        Some("boolean") => ParameterKind::Boolean,
        Some("integer") => ParameterKind::Number,
        _ => ParameterKind::Text,
    }
}

fn parameter_name(spot: Spot<'_>, wire: &str) -> String {
    assert!(!wire.is_empty(), "a parameter is named");

    let identifier = spot.identifier();

    let fixed = PARAMETER_NAMES
        .iter()
        .find(|(operation, named, _name)| *operation == identifier && *named == wire);

    let name =
        fixed.map_or_else(|| field_name(wire), |(_operation, _named, name)| (*name).to_owned());

    assert!(!name.is_empty(), "a parameter field is named");

    name
}

fn parameter_read<'document>(
    registry: &Registry<'document>,
    spot: Spot<'document>,
    raw: &'document Value,
) -> Option<Parameter> {
    assert!(registry.document.is_object(), "a registry reads a document");

    let resolved = registry.resolved(raw);
    let wire = resolved.get("name").and_then(Value::as_str).unwrap_or_default();
    let place = resolved.get("in").and_then(Value::as_str).unwrap_or_default();

    assert!(!wire.is_empty(), "a parameter is named");

    if wire.eq_ignore_ascii_case("Authorization") {
        return None;
    }

    if header_is_xsrf(resolved) {
        return None;
    }

    let location = match place {
        "header" => Location::Header,
        "path" => Location::Path,
        "query" => Location::Query,
        _ => unknown_location(wire),
    };

    let is_required = resolved.get("required").and_then(Value::as_bool).unwrap_or_default();
    let kind = parameter_text(spot, wire, parameter_kind(registry, resolved));

    if location == Location::Header {
        assert!(!is_required, "the required header {wire} needs a call the emitter lacks");
        assert_eq!(kind, ParameterKind::Text, "the header {wire} carries one text");
    }

    Some(Parameter {
        is_required: is_required || location == Location::Path,
        kind,
        location,
        name: parameter_name(spot, wire),
        wire: wire.to_owned(),
    })
}

fn parameter_text(spot: Spot<'_>, wire: &str, declared: ParameterKind) -> ParameterKind {
    assert!(!wire.is_empty(), "a parameter is named");

    let identifier = spot.identifier();

    let is_text =
        PARAMETER_TEXTS.iter().any(|(operation, named)| *operation == identifier && *named == wire);

    if is_text {
        assert_eq!(declared, ParameterKind::Number, "a text override fixes a declared number");

        return ParameterKind::Text;
    }

    declared
}

fn parameters_read<'document>(
    registry: &Registry<'document>,
    spot: Spot<'document>,
) -> Vec<Parameter> {
    assert!(spot.operation.is_object(), "an operation is an object");
    assert!(spot.item.is_object(), "a path item is an object");

    let mut by_wire: BTreeMap<(String, u32), Parameter> = BTreeMap::new();

    for raw in spot.parameters() {
        if let Some(parameter) = parameter_read(registry, spot, raw) {
            let place = location_rank(parameter.location);
            let _replaced = by_wire.insert((parameter.wire.clone(), place), parameter);
        }
    }

    by_wire.into_values().collect()
}

fn placeholders_check(path: &str, parameters: &[Parameter]) -> Result<(), String> {
    assert!(path.starts_with('/'), "a path is rooted at its base");

    let wires = parameters.iter().filter(|found| found.location == Location::Path);

    for parameter in wires {
        if !path.contains(&format!("{{{}}}", parameter.wire)) {
            return Err(format!("{path} never names the path parameter {}", parameter.wire));
        }
    }

    assert!(!path.contains('?'), "a query term belongs in the parameters");

    Ok(())
}

fn placeholders_declare(path: &str, parameters: &mut Vec<Parameter>) {
    assert!(path.starts_with('/'), "a path is rooted at its base");

    for piece in path.split('{').skip(1) {
        let wire = piece.split('}').next().unwrap_or_default();

        assert!(!wire.is_empty(), "a placeholder is named");

        if !parameters.iter().any(|found| found.wire == wire) {
            parameters.push(Parameter {
                is_required: true,
                kind: ParameterKind::Text,
                location: Location::Path,
                name: field_name(wire),
                wire: wire.to_owned(),
            });
        }
    }
}

fn record_is(node: &Value) -> bool {
    assert!(node.is_object() || node.is_boolean(), "a schema is an object or a boolean");

    let properties = node.get("properties").and_then(Value::as_object);
    let members = node.get("allOf").and_then(Value::as_array);

    if properties.is_some_and(|found| !found.is_empty()) {
        return true;
    }

    assert!(properties.is_none_or(Map::is_empty), "a record with members already passed");

    members.is_some_and(|found| found.len() >= 2)
}

fn reply_is_binary<'document>(registry: &Registry<'document>, spot: Spot<'document>) -> bool {
    assert!(spot.operation.is_object(), "an operation is an object");

    if BINARY_REPLIES.contains(&spot.identifier()) {
        return true;
    }

    let Some(responses) = spot.operation.get("responses").and_then(Value::as_object) else {
        return false;
    };

    let contents = responses_success(responses)
        .map(|(_status, response)| registry.resolved(response))
        .filter_map(|response| response.get("content").and_then(Value::as_object));

    let named: Vec<&Map<String, Value>> = contents.filter(|found| !found.is_empty()).collect();

    if named.is_empty() {
        return false;
    }

    !named.into_iter().flatten().any(|(media, node)| {
        let schema = node.get("schema").map(|found| registry.resolved(found));

        media_is_json(media, schema)
    })
}

fn reply_kind<'document>(
    registry: &mut Registry<'document>,
    spot: Spot<'document>,
    reply: Option<&'document Value>,
    name: &str,
) -> String {
    assert!(!name.is_empty(), "an operation is named");
    assert!(spot.operation.is_object(), "an operation is an object");

    if let Some(node) = reply {
        return registry.kind(node, &format!("{name}Reply"));
    }

    let Some(responses) = spot.operation.get("responses").and_then(Value::as_object) else {
        return KIND_NOTHING.to_owned();
    };

    let Some(status) = status_success(responses) else {
        return KIND_NOTHING.to_owned();
    };

    let response = responses.get(status).map(|found| registry.resolved(found));
    let content = response.and_then(|found| found.get("content")).and_then(Value::as_object);

    if content.is_none_or(Map::is_empty) {
        return KIND_NOTHING.to_owned();
    }

    if status == STATUS_NO_CONTENT {
        return KIND_NOTHING.to_owned();
    }

    KIND_ANY.to_owned()
}

fn reply_schema<'document>(
    registry: &Registry<'document>,
    operation: &'document Value,
) -> Option<&'document Value> {
    assert!(operation.is_object(), "an operation is an object");

    let responses = operation.get("responses").and_then(Value::as_object)?;

    for (_status, response) in responses_success(responses) {
        let content = registry.resolved(response).get("content").and_then(Value::as_object);

        for (media, node) in content.into_iter().flatten() {
            let schema = node.get("schema").filter(|named| schema_names_something(named));
            let resolved = schema.map(|found| registry.resolved(found));

            if resolved.is_some_and(|found| media_is_json(media, Some(found))) {
                return schema;
            }
        }
    }

    assert!(!responses.is_empty(), "an operation names its responses");

    None
}

fn responses_success(responses: &Map<String, Value>) -> impl Iterator<Item = (&String, &Value)> {
    assert!(!responses.is_empty(), "an operation names its responses");

    responses.iter().filter(|(status, _response)| status.starts_with(['2', '3']))
}

fn schema_names_something(schema: &Value) -> bool {
    assert!(schema.is_object() || schema.is_boolean(), "a schema is an object or a boolean");

    schema.as_object().is_some_and(|found| !found.is_empty())
}

fn status_success(responses: &Map<String, Value>) -> Option<&str> {
    let statuses = responses.keys().map(String::as_str);
    let found = statuses.filter(|status| status.starts_with(['2', '3'])).min();

    assert!(found.is_none_or(|status| !status.is_empty()), "a status is never empty");
    assert!(found.is_none_or(|status| responses.contains_key(status)), "a status is a response");

    found
}

fn step_inner<'document>(step: &Step<'document>, current: &'document Value) -> &'document Value {
    assert!(current.is_object() || current.is_boolean(), "a schema is an object or a boolean");

    match *step {
        Step::Follow(inner) | Step::Wrap(_, inner) => inner,
        Step::Leaf(_) => current,
    }
}

fn step_of_type(node: &Value) -> Step<'_> {
    assert!(node.is_object() || node.is_boolean(), "a schema is an object or a boolean");

    match node.get("type").and_then(Value::as_str) {
        Some("array") => node.get("items").map_or_else(
            || Step::Leaf(format!("crate::List<{KIND_ANY}>")),
            |items| Step::Wrap("List", items),
        ),
        Some("boolean") => Step::Leaf("crate::Boolean".to_owned()),
        Some("integer") => Step::Leaf("crate::Integer".to_owned()),
        Some("number") => Step::Leaf("crate::Number".to_owned()),
        Some("string") => Step::Leaf("crate::Text".to_owned()),
        _ => match node.get("additionalProperties") {
            Some(inner) if inner.is_object() => Step::Wrap("Map", inner),
            Some(Value::Bool(true)) => Step::Leaf(format!("crate::Map<{KIND_ANY}>")),
            _ => Step::Leaf(KIND_ANY.to_owned()),
        },
    }
}

fn token_field_find(properties: &[(&str, &Value)]) -> Option<String> {
    assert!(!TOKEN_FIELDS.is_empty(), "a token field has a spelling");

    for field in TOKEN_FIELDS {
        if properties.iter().any(|(wire, _node)| *wire == field) {
            return Some(field.to_owned());
        }
    }

    assert!(TOKEN_FIELDS.iter().all(|field| !field.is_empty()), "a token field is spelled");

    None
}

fn unknown_location(wire: &str) -> Location {
    assert!(!wire.is_empty(), "a parameter is named");
    assert!(wire.is_empty(), "{wire} sits in a place a request cannot carry");

    Location::Query
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::{
        Location,
        Parameter,
        ParameterKind,
        Pending,
        Registry,
        body_read,
        media_is_json,
        member_constant,
        page_source_link,
        page_source_token,
        parameter_kind,
        placeholders_check,
        placeholders_declare,
        record_is,
        step_of_type,
    };

    #[test]
    fn a_body_behind_a_reference_is_read_through_it() {
        let document = json!({"components": {
            "requestBodies": {"Page": {"required": true, "content": {"application/json": {}}}},
            "schemas": {},
        }});

        let registry = Registry::new(&document);
        let operation = json!({"requestBody": {"$ref": "#/components/requestBodies/Page"}});
        let body = body_read(&registry, &operation).expect("the reference names a body");

        assert!(body.is_required, "the body's own flag survives the reference");
        assert_eq!(body.kind, super::BodyKind::JSONValue, "a JSON body");
    }

    #[test]
    fn a_clashing_member_is_renamed_only_in_the_schema_that_holds_the_clash() {
        let node = json!({});
        let clashing = Pending { name: "LookAndFeelWithLinks".to_owned(), nodes: vec![&node] };
        let plain = Pending { name: "Content".to_owned(), nodes: vec![&node] };

        assert_eq!(member_constant(&clashing, "_links"), "HYPERMEDIA_LINKS", "the clash");
        assert_eq!(member_constant(&clashing, "links"), "LINKS", "the member it clashed with");
        assert_eq!(member_constant(&plain, "_links"), "LINKS", "the same wire elsewhere");
    }

    #[test]
    fn a_comma_list_a_deep_object_and_a_repeated_list_are_told_apart() {
        let document = json!({"components": {"schemas": {}}});
        let registry = Registry::new(&document);
        let array = json!({"type": "array", "items": {"type": "string"}});
        let map = json!({"type": "object", "additionalProperties": true});
        let joined = json!({"name": "expand", "in": "query", "explode": false, "schema": array});
        let repeated = json!({"name": "status", "in": "query", "schema": array});
        let deep = json!({"name": "filter", "in": "query", "style": "deepObject", "schema": map});

        assert_eq!(parameter_kind(&registry, &joined), ParameterKind::TextsJoined, "a comma list");
        assert_eq!(parameter_kind(&registry, &repeated), ParameterKind::Texts, "a repeated name");
        assert_eq!(parameter_kind(&registry, &deep), ParameterKind::Pairs, "bracketed members");
    }

    #[test]
    fn a_json_media_type_is_known_by_its_essence_and_a_wildcard_by_its_schema() {
        let record = json!({"properties": {"modules": {}}});
        let text = json!({"type": "string"});

        assert!(media_is_json("application/json", None), "the plain type");
        assert!(media_is_json("application/json;charset=UTF-8", None), "with a parameter");
        assert!(media_is_json("*/*", Some(&record)), "a wildcard over a record");
        assert!(!media_is_json("*/*", Some(&text)), "a wildcard over a string");
        assert!(!media_is_json("*/*", None), "a wildcard over nothing");
        assert!(!media_is_json("text/csv", Some(&record)), "another type");
    }

    #[test]
    fn a_kind_peels_lists_and_maps_down_to_a_leaf() {
        let document = json!({"components": {"schemas": {}}});
        let mut registry = Registry::new(&document);

        let lists =
            json!({"type": "array", "items": {"type": "array", "items": {"type": "string"}}});

        let map = json!({"type": "object", "additionalProperties": {"type": "integer"}});
        let bare = json!({});

        assert_eq!(registry.kind(&lists, "X"), "crate::List<crate::List<crate::Text>>", "nested");
        assert_eq!(registry.kind(&map, "X"), "crate::Map<crate::Integer>", "a map");
        assert_eq!(registry.kind(&bare, "X"), "crate::Any", "an empty schema");
    }

    #[test]
    fn a_link_page_is_found_in_either_container_and_an_open_map_counts() {
        let declared = json!({"properties": {"next": {"type": "string"}}});
        let open = json!({"type": "object", "additionalProperties": {"type": "string"}});
        let closed = json!({"properties": {"base": {"type": "string"}}});
        let declared_next = link_next("links", &declared);
        let open_next = link_next("_links", &open);
        let declared_link = "crate::Next::Link(\"/links/next\")";
        let open_link = "crate::Next::Link(\"/_links/next\")";

        assert_eq!(declared_next.as_deref(), Some(declared_link), "a declared next");
        assert_eq!(open_next.as_deref(), Some(open_link), "an open map");
        assert_eq!(link_next("_links", &closed), None, "a container that names no next");
    }

    #[test]
    #[should_panic(expected = "disagree on a kind")]
    fn a_member_merged_schemas_define_as_two_kinds_is_refused() {
        let document = json!({"components": {"schemas": {}}});
        let text = json!({"type": "object", "properties": {"id": {"type": "string"}}});
        let number = json!({"type": "object", "properties": {"id": {"type": "integer"}}});
        let reply = json!({"allOf": [text, number]});
        let mut registry = Registry::new(&document);

        assert_eq!(registry.kind(&reply, "Reply"), "crate::Object<SCHEMA::Reply>", "a record");

        registry.drain();
    }

    #[test]
    fn a_member_two_merged_schemas_define_becomes_one_record_holding_both() {
        let document = json!({"components": {"schemas": {
            "Links": {"type": "object", "properties": {"webui": {"type": "string"}}},
            "Page": {"type": "object", "properties": {
                "_links": {"$ref": "#/components/schemas/Links"},
                "id": {"type": "string"},
            }},
        }}});

        let added = json!({"type": "object", "properties": {
            "_links": {"type": "object", "properties": {"base": {"type": "string"}}},
            "id": {"type": "string", "description": "the same kind, described"},
        }});

        let reply = json!({"allOf": [{"$ref": "#/components/schemas/Page"}, added]});
        let mut registry = Registry::new(&document);
        let kind = registry.kind(&reply, "GetPageReply");

        registry.drain();

        let links = "crate::Object<SCHEMA::GetPageReplyLinks>".to_owned();
        let text = "crate::Text".to_owned();
        let reply_fields = [("_links".to_owned(), links), ("id".to_owned(), text.clone())];
        let link_fields = [("base".to_owned(), text.clone()), ("webui".to_owned(), text.clone())];
        let own_fields = [("webui".to_owned(), text)];

        assert_eq!(kind, "crate::Object<SCHEMA::GetPageReply>", "the reply is a record");
        assert_eq!(fields_of(&registry, "GetPageReply"), reply_fields, "the merged reply");
        assert_eq!(fields_of(&registry, "GetPageReplyLinks"), link_fields, "both definitions");
        assert_eq!(fields_of(&registry, "Links"), own_fields, "the shared schema keeps its own");
    }

    #[test]
    fn a_path_parameter_the_template_never_names_is_refused() {
        let stray = Parameter {
            is_required: true,
            kind: ParameterKind::Text,
            location: Location::Path,
            name: "id".to_owned(),
            wire: "id".to_owned(),
        };

        assert!(placeholders_check("/pages", &[stray]).is_err(), "nothing could fill it");
    }

    #[test]
    fn a_placeholder_nobody_declared_becomes_a_required_parameter() {
        let path = "/wiki/api/v2/pages/{id}/footer-comments";
        let mut parameters: Vec<Parameter> = Vec::new();

        placeholders_declare(path, &mut parameters);

        assert_eq!(parameters.len(), 1, "the placeholder is declared");
        assert_eq!(parameters[0].wire, "id", "under its wire name");
        assert_eq!(parameters[0].location, Location::Path, "in the path");
        assert!(parameters[0].is_required, "and required");
        assert!(placeholders_check(path, &parameters).is_ok(), "the path is now satisfied");
    }

    #[test]
    fn a_reference_to_a_record_stays_a_reference_and_an_alias_is_followed() {
        let document = json!({"components": {"schemas": {
            "Names": {"type": "array", "items": {"type": "string"}},
            "User": {"type": "object", "properties": {"name": {"type": "string"}}},
        }}});

        let mut registry = Registry::new(&document);
        let record = json!({"$ref": "#/components/schemas/User"});
        let alias = json!({"$ref": "#/components/schemas/Names"});

        assert_eq!(registry.kind(&record, "X"), "crate::Object<SCHEMA::User>", "a record");
        assert_eq!(registry.kind(&alias, "X"), "crate::List<crate::Text>", "an alias");
    }

    #[test]
    fn a_token_page_needs_both_its_token_and_its_items() {
        let document = json!({"components": {"schemas": {}}});
        let registry = Registry::new(&document);
        let results = json!({"type": "array", "items": {"type": "string"}});
        let cursor = json!({"type": "string"});

        let whole = page_source_token(&registry, &[("cursor", &cursor), ("results", &results)]);
        let bare = page_source_token(&registry, &[("cursor", &cursor)]);
        let token = "crate::Next::Token(\"/cursor\")";

        assert_eq!(whole.map(|found| found.next).as_deref(), Some(token), "a token page");
        assert!(bare.is_none(), "a token with nothing to walk");
    }

    #[test]
    fn an_inline_record_is_registered_once_under_the_name_of_where_it_sits() {
        let document = json!({"components": {"schemas": {}}});
        let mut registry = Registry::new(&document);
        let inline = json!({"type": "object", "properties": {"id": {"type": "integer"}}});

        assert_eq!(registry.kind(&inline, "Space"), "crate::Object<SCHEMA::Space>", "named");
        assert_eq!(registry.kind(&inline, "Space"), "crate::Object<SCHEMA::Space>", "named again");

        registry.drain();

        assert_eq!(registry.schemas.len(), 1, "one schema, however often it was seen");
    }

    fn fields_of(registry: &Registry<'_>, name: &str) -> Vec<(String, String)> {
        assert!(!name.is_empty(), "a schema is named");

        let found = registry.schemas.get(name).expect("a generated schema");

        let mut pairs: Vec<(String, String)> =
            found.iter().map(|field| (field.wire.clone(), field.kind.clone())).collect();

        pairs.sort();

        assert_eq!(pairs.len(), found.len(), "each field is listed once");

        pairs
    }

    fn link_next(container: &str, links: &Value) -> Option<String> {
        assert!(!container.is_empty(), "a link container is named");
        assert!(links.is_object(), "a link container is a schema");

        let document = json!({"components": {"schemas": {}}});
        let registry = Registry::new(&document);
        let results = json!({"type": "array", "items": {"type": "string"}});
        let properties = [(container, links), ("results", &results)];

        page_source_link(&registry, &properties).map(|found| found.next)
    }

    #[test]
    fn only_a_schema_with_members_is_a_record() {
        assert!(record_is(&json!({"properties": {"a": {}}})), "members");
        assert!(!record_is(&json!({"properties": {}})), "no members");
        assert!(!record_is(&json!({"type": "object"})), "no members");
        assert!(record_is(&json!({"allOf": [{}, {}]})), "a merge of two schemas");
        assert!(matches!(step_of_type(&json!({"type": "string"})), super::Step::Leaf(_)), "a leaf");
    }
}
