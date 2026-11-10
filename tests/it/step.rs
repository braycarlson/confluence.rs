use alloc::collections::BTreeMap;
use core::cell::RefCell;
use std::io::Cursor;

use confluence::{Binary, Client, Error, Kind, Operation, Paged, Part, Reply, Transport};
use serde_json::{Map, Value};

pub(crate) const WIRE_BODY: &str = "{body}";
pub(crate) const WIRE_PARTS: &str = "{parts}";
const PAGE_COUNT_MAX: u64 = 64;
const PART_COUNT_MAX: usize = 8;
const READ_COUNT_MAX: usize = VALUE_COUNT_MAX + 2;
const VALUE_COUNT_MAX: usize = 32;

pub(crate) type Run<Network> = fn(&Client<Network>, &Step<'_>) -> Result<Sent, Error>;

#[derive(Debug)]
enum Held<'plan> {
    Boolean(bool),
    Empty,
    Number(i64),
    Numbers(Vec<i64>),
    Pairs(Vec<(&'plan str, &'plan str)>),
    Text(&'plan str),
    Texts(Vec<&'plan str>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Need {
    Optional,
    Required,
}

#[derive(Debug)]
pub(crate) struct Sent {
    pub bytes: Vec<u8>,
    pub is_typed: bool,
    pub item_count: u64,
    pub location: Option<String>,
    pub pages: Vec<Value>,
    pub value: Value,
}

#[derive(Debug)]
pub(crate) struct Step<'plan> {
    body: Option<&'plan Value>,
    identifier: &'plan str,
    page_count_max: Option<u64>,
    parts: Option<Vec<Part<'plan>>>,
    reads: RefCell<Vec<(&'plan str, Need)>>,
    values: BTreeMap<&'plan str, Held<'plan>>,
}

impl Sent {
    fn of_reply<Body>(reply: Reply<Body>) -> Self
    where
        Body: Kind,
    {
        assert!(reply.location().is_none_or(|found| !found.is_empty()), "a location names a place");

        let is_typed = reply.read().is_some();
        let location = reply.location().map(str::to_owned);
        let value = reply.into_value();

        if Body::IS_BODY_IGNORED {
            assert!(value.is_null(), "a reply of nothing keeps nothing");
            assert!(is_typed, "a reply of nothing always reads");
        }

        Self { bytes: Vec::new(), is_typed, item_count: 0, location, pages: Vec::new(), value }
    }
}

impl<'plan> Step<'plan> {
    pub(crate) fn body(&self) -> Option<&'plan Value> {
        assert!(!self.identifier.is_empty(), "a step names its operation");

        if self.body.is_some() {
            self.read_push(WIRE_BODY, Need::Optional);
        }

        assert_eq!(self.body.is_some(), self.is_read(WIRE_BODY), "a held body is read");

        self.body
    }

    pub(crate) fn body_required(&self) -> &'plan Value {
        assert!(!self.identifier.is_empty(), "a step names its operation");

        let Some(body) = self.body else {
            panic!("the step for {} holds no body", self.identifier);
        };

        self.read_push(WIRE_BODY, Need::Required);

        assert!(self.is_read(WIRE_BODY), "the body is read");

        body
    }

    pub(crate) fn boolean(&self, wire: &str) -> Option<bool> {
        assert!(!wire.is_empty(), "a value is named");

        let held = self.held(wire, Need::Optional)?;

        let Held::Boolean(found) = held else {
            panic!("the step for {} holds no boolean at {wire}", self.identifier);
        };

        assert!(self.is_read(wire), "a boolean that was taken is read");

        Some(*found)
    }

    pub(crate) fn download<Network, Called>(
        &self,
        client: &Client<Network>,
        operation: &Called,
        options: &Called::Options<'_>,
    ) -> Result<Sent, Error>
    where
        Network: Transport,
        Called: Operation<Reply = Binary>,
    {
        if self.is_walk() {
            return Err(Error::Input(format!("{} is no walk", self.identifier)));
        }

        let mut sink = Cursor::new(Vec::new());
        let written = client.download_with(operation, options, &mut sink)?;
        let kept = usize::try_from(written).unwrap_or(usize::MAX);
        let mut bytes = sink.into_inner();

        bytes.truncate(kept);

        assert!(bytes.len() <= kept, "only the bytes the download counted are kept");
        assert!(Called::IS_READ, "a download only reads");

        Ok(Sent {
            bytes,
            is_typed: true,
            item_count: 0,
            location: None,
            pages: Vec::new(),
            value: Value::Null,
        })
    }

    fn held(&self, wire: &str, need: Need) -> Option<&Held<'plan>> {
        assert!(!wire.is_empty(), "a value is named");
        assert!(self.values.len() <= VALUE_COUNT_MAX, "a step holds few values");

        let (key, held) = self.values.get_key_value(wire)?;

        self.read_push(key, need);

        Some(held)
    }

    fn held_required(&self, wire: &str) -> &Held<'plan> {
        assert!(!wire.is_empty(), "a value is named");

        let Some(held) = self.held(wire, Need::Required) else {
            panic!("the step for {} holds no {wire}, which the operation needs", self.identifier);
        };

        assert!(self.is_read(wire), "a required value that was taken is read");

        held
    }

    fn held_wires(&self) -> Vec<&'plan str> {
        let mut wires: Vec<&'plan str> = self.values.keys().copied().collect();

        if self.body.is_some() {
            wires.push(WIRE_BODY);
        }

        if self.parts.is_some() {
            wires.push(WIRE_PARTS);
        }

        wires.sort_unstable();

        assert!(wires.len() >= self.values.len(), "a body and its parts only add to the count");
        assert!(wires.len() <= READ_COUNT_MAX, "a step holds one body at most");

        wires
    }

    pub(crate) fn identifier(&self) -> &'plan str {
        assert!(!self.identifier.is_empty(), "a step names its operation");
        assert!(self.values.len() <= VALUE_COUNT_MAX, "a step holds few values");

        self.identifier
    }

    fn is_read(&self, wire: &str) -> bool {
        assert!(!wire.is_empty(), "a value is named");

        let reads = self.reads.borrow();

        assert!(reads.len() <= READ_COUNT_MAX, "a step is read finitely");

        reads.iter().any(|(read, _need)| *read == wire)
    }

    pub(crate) fn is_spent(&self) -> bool {
        let held = self.held_wires();
        let mut read: Vec<&str> = self.reads.borrow().iter().map(|(wire, _need)| *wire).collect();

        read.sort_unstable();

        assert!(held.windows(2).all(|pair| pair.first() != pair.last()), "each value is held once");
        assert!(read.len() <= READ_COUNT_MAX, "a step is read finitely");

        read == held
    }

    pub(crate) fn is_walk(&self) -> bool {
        assert!(!self.identifier.is_empty(), "a step names its operation");
        assert!(self.page_count_max.is_none_or(|count| count >= 1), "a walk reads a page");

        self.page_count_max.is_some()
    }

    pub(crate) fn number(&self, wire: &str) -> Option<i64> {
        assert!(!wire.is_empty(), "a value is named");

        let held = self.held(wire, Need::Optional)?;

        let Held::Number(found) = held else {
            panic!("the step for {} holds no number at {wire}", self.identifier);
        };

        assert!(self.is_read(wire), "a number that was taken is read");

        Some(*found)
    }

    pub(crate) fn number_required(&self, wire: &str) -> i64 {
        assert!(!wire.is_empty(), "a value is named");

        let held = self.held_required(wire);

        let Held::Number(found) = held else {
            panic!("the step for {} holds no number at {wire}", self.identifier);
        };

        assert!(self.is_read(wire), "a number that was taken is read");

        *found
    }

    pub(crate) fn numbers(&self, wire: &str) -> Option<&[i64]> {
        assert!(!wire.is_empty(), "a value is named");

        let found = match self.held(wire, Need::Optional)? {
            Held::Numbers(numbers) => numbers.as_slice(),
            Held::Empty => &[],
            Held::Boolean(_)
            | Held::Number(_)
            | Held::Pairs(_)
            | Held::Text(_)
            | Held::Texts(_) => {
                panic!("the step for {} holds no list of numbers at {wire}", self.identifier)
            }
        };

        assert!(found.len() <= VALUE_COUNT_MAX, "a list holds few numbers");

        Some(found)
    }

    pub(crate) fn of(plan: &'plan Value) -> Result<Self, Error> {
        let identifier = plan.get("identifier").and_then(Value::as_str).unwrap_or_default();
        let page_count_max = plan.get("walk").and_then(Value::as_u64);

        if identifier.is_empty() {
            return Err(Error::Input("a step names no operation".to_owned()));
        }

        if page_count_max.is_some_and(|count| count == 0) {
            return Err(Error::Input(format!("the walk of {identifier} reads no page")));
        }

        let step = Self {
            body: plan.get("body").filter(|found| !found.is_null()),
            identifier,
            page_count_max,
            parts: match plan.get("parts") {
                None | Some(Value::Null) => None,
                Some(found) => Some(parts_of(found)?),
            },
            reads: RefCell::new(Vec::new()),
            values: match plan.get("values") {
                None | Some(Value::Null) => BTreeMap::new(),
                Some(Value::Object(members)) if members.len() <= VALUE_COUNT_MAX => {
                    values_of(members)?
                }
                Some(_) => {
                    return Err(Error::Input(format!(
                        "the values of {identifier} are no object of at most {VALUE_COUNT_MAX}"
                    )));
                }
            },
        };

        assert!(step.values.len() <= VALUE_COUNT_MAX, "a step holds few values");
        assert!(step.reads.borrow().is_empty(), "a fresh step has read nothing");

        Ok(step)
    }

    pub(crate) fn pairs(&self, wire: &str) -> Option<&[(&'plan str, &'plan str)]> {
        assert!(!wire.is_empty(), "a value is named");

        let found = match self.held(wire, Need::Optional)? {
            Held::Pairs(pairs) => pairs.as_slice(),
            Held::Empty => &[],
            Held::Boolean(_)
            | Held::Number(_)
            | Held::Numbers(_)
            | Held::Text(_)
            | Held::Texts(_) => {
                panic!("the step for {} holds no map of texts at {wire}", self.identifier)
            }
        };

        assert!(found.len() <= VALUE_COUNT_MAX, "a map holds few pairs");

        Some(found)
    }

    pub(crate) fn parts_required(&self) -> &[Part<'plan>] {
        assert!(!self.identifier.is_empty(), "a step names its operation");

        let Some(parts) = self.parts.as_deref() else {
            panic!("the step for {} holds no parts", self.identifier);
        };

        self.read_push(WIRE_PARTS, Need::Required);

        assert!(!parts.is_empty(), "a multipart body holds a part");

        parts
    }

    fn read_push(&self, wire: &'plan str, need: Need) {
        assert!(!wire.is_empty(), "a read value is named");

        let mut reads = self.reads.borrow_mut();

        reads.push((wire, need));

        assert!(reads.len() <= READ_COUNT_MAX, "a step is read finitely");
    }

    pub(crate) fn required(&self) -> Vec<&'plan str> {
        let reads = self.reads.borrow();
        let mut required: Vec<&'plan str> = Vec::new();

        for (wire, need) in reads.iter() {
            if *need == Need::Required {
                required.push(wire);
            }
        }

        required.sort_unstable();

        assert!(required.len() <= reads.len(), "a required read is a read");
        assert!(required.windows(2).all(|pair| pair.first() != pair.last()), "each is read once");

        required
    }

    pub(crate) fn send<Network, Called>(
        &self,
        client: &Client<Network>,
        operation: &Called,
        options: &Called::Options<'_>,
    ) -> Result<Sent, Error>
    where
        Network: Transport,
        Called: Operation,
        Called::Reply: Kind,
    {
        if self.is_walk() {
            return Err(Error::Input(format!("{} is no walk", self.identifier)));
        }

        let reply = client.send_with(operation, options)?;

        assert!(self.is_spent(), "a step is spent before it sends");

        Ok(Sent::of_reply(reply))
    }

    pub(crate) fn send_bare<Network, Called>(
        &self,
        client: &Client<Network>,
        operation: &Called,
    ) -> Result<Sent, Error>
    where
        Network: Transport,
        Called: Operation,
        Called::Reply: Kind,
    {
        if self.is_walk() {
            return Err(Error::Input(format!("{} is no walk", self.identifier)));
        }

        let reply = client.send(operation)?;

        assert!(self.is_spent(), "a step is spent before it sends");

        Ok(Sent::of_reply(reply))
    }

    pub(crate) fn text(&self, wire: &str) -> Option<&'plan str> {
        assert!(!wire.is_empty(), "a value is named");

        let held = self.held(wire, Need::Optional)?;

        let Held::Text(found) = held else {
            panic!("the step for {} holds no text at {wire}", self.identifier);
        };

        assert!(self.is_read(wire), "a text that was taken is read");

        Some(*found)
    }

    pub(crate) fn text_required(&self, wire: &str) -> &'plan str {
        assert!(!wire.is_empty(), "a value is named");

        let held = self.held_required(wire);

        let Held::Text(found) = held else {
            panic!("the step for {} holds no text at {wire}", self.identifier);
        };

        assert!(self.is_read(wire), "a text that was taken is read");

        found
    }

    pub(crate) fn texts(&self, wire: &str) -> Option<&[&'plan str]> {
        assert!(!wire.is_empty(), "a value is named");

        let found = match self.held(wire, Need::Optional)? {
            Held::Texts(texts) => texts.as_slice(),
            Held::Empty => &[],
            Held::Boolean(_)
            | Held::Number(_)
            | Held::Numbers(_)
            | Held::Pairs(_)
            | Held::Text(_) => {
                panic!("the step for {} holds no list of texts at {wire}", self.identifier)
            }
        };

        assert!(found.len() <= VALUE_COUNT_MAX, "a list holds few texts");

        Some(found)
    }

    pub(crate) fn texts_required(&self, wire: &str) -> &[&'plan str] {
        assert!(!wire.is_empty(), "a value is named");

        let found = match self.held_required(wire) {
            Held::Texts(texts) => texts.as_slice(),
            Held::Empty => &[],
            Held::Boolean(_)
            | Held::Number(_)
            | Held::Numbers(_)
            | Held::Pairs(_)
            | Held::Text(_) => {
                panic!("the step for {} holds no list of texts at {wire}", self.identifier)
            }
        };

        assert!(found.len() <= VALUE_COUNT_MAX, "a list holds few texts");

        found
    }

    pub(crate) fn walk<'call, Network, Walked>(
        &self,
        client: &Client<Network>,
        operation: &'call Walked,
        options: Walked::Options<'call>,
    ) -> Result<Sent, Error>
    where
        Network: Transport,
        Walked: Paged,
    {
        let page_count_max = self.page_count_max.unwrap_or(1).min(PAGE_COUNT_MAX);
        let page_budget = usize::try_from(page_count_max).unwrap_or(1);
        let mut pages: Vec<Value> = Vec::new();
        let mut is_typed = true;
        let mut item_count = 0_u64;

        for walked in client.walk_with(operation, options).take(page_budget) {
            let page = walked?;
            let typed_count = u64::try_from(page.items().count()).unwrap_or(u64::MAX);

            is_typed = is_typed && typed_count == page.count();
            item_count = item_count.strict_add(page.count());
            pages.push(page.value().clone());
        }

        assert!(pages.len() <= page_budget, "a walk stays inside its page budget");
        assert!(page_budget >= 1, "a walk reads at least one page");

        Ok(Sent {
            bytes: Vec::new(),
            is_typed,
            item_count,
            location: None,
            pages,
            value: Value::Null,
        })
    }
}

fn held_of(value: &Value) -> Option<Held<'_>> {
    let held = match value {
        Value::Array(items) if items.is_empty() => Held::Empty,
        Value::Array(items) => items_held(items)?,
        Value::Bool(found) => Held::Boolean(*found),
        Value::Number(found) => Held::Number(found.as_i64()?),
        Value::Object(members) => Held::Pairs(pairs_of(members)?),
        Value::String(found) => Held::Text(found),
        Value::Null => return None,
    };

    assert!(!matches!(&held, Held::Texts(found) if found.is_empty()), "an empty list is Empty");
    assert!(!matches!(&held, Held::Numbers(found) if found.is_empty()), "an empty list is Empty");

    Some(held)
}

fn items_held(items: &[Value]) -> Option<Held<'_>> {
    assert!(!items.is_empty(), "an empty list is held as Empty");

    if items.iter().all(Value::is_string) {
        return Some(Held::Texts(items.iter().filter_map(Value::as_str).collect()));
    }

    let numbers: Vec<i64> = items.iter().filter_map(Value::as_i64).collect();

    if numbers.len() == items.len() {
        return Some(Held::Numbers(numbers));
    }

    assert!(numbers.len() < items.len(), "a mixed list holds something that is no number");

    None
}

fn member_text<'raw>(members: &'raw Map<String, Value>, name: &str) -> Option<&'raw str> {
    assert!(!name.is_empty(), "a member is named");
    assert!(members.len() <= PART_COUNT_MAX, "a part is described by a few members");

    members.get(name).and_then(Value::as_str)
}

fn pairs_of(members: &Map<String, Value>) -> Option<Vec<(&str, &str)>> {
    assert!(members.len() <= VALUE_COUNT_MAX, "a map holds few pairs");

    let pairs: Vec<(&str, &str)> =
        members.iter().filter_map(|(key, value)| Some((key.as_str(), value.as_str()?))).collect();

    if pairs.len() != members.len() {
        return None;
    }

    assert!(pairs.iter().all(|(key, _value)| !key.is_empty()), "a pair is keyed");

    Some(pairs)
}

fn part_of(members: &Map<String, Value>) -> Result<Part<'_>, Error> {
    assert!(members.len() <= PART_COUNT_MAX, "a part is described by a few members");

    let name = member_text(members, "name").unwrap_or_default();

    let Some(bytes) = member_text(members, "text").map(str::as_bytes) else {
        return Err(Error::Input(format!("the part {name} holds no text")));
    };

    let part = Part {
        bytes,
        content_type: member_text(members, "content_type").unwrap_or("text/plain"),
        file_name: member_text(members, "file_name"),
        name,
    };

    assert!(!part.content_type.is_empty(), "a part names its type");
    assert!(members.contains_key("text"), "a part carries its text");

    Ok(part)
}

fn parts_of(raw: &Value) -> Result<Vec<Part<'_>>, Error> {
    assert!(!raw.is_null(), "a step without parts never reads them");

    let listed = raw.as_array().ok_or_else(|| Error::Input("parts are a list".to_owned()))?;

    if listed.len() > PART_COUNT_MAX {
        return Err(Error::Input(format!("a step holds at most {PART_COUNT_MAX} parts")));
    }

    let mut parts: Vec<Part<'_>> = Vec::with_capacity(listed.len());

    for item in listed {
        let Some(members) = item.as_object().filter(|found| found.len() <= PART_COUNT_MAX) else {
            return Err(Error::Input("a part is described by an object of few members".to_owned()));
        };

        parts.push(part_of(members)?);
    }

    assert_eq!(parts.len(), listed.len(), "each listed part is a part");

    Ok(parts)
}

fn values_of(members: &Map<String, Value>) -> Result<BTreeMap<&str, Held<'_>>, Error> {
    assert!(members.len() <= VALUE_COUNT_MAX, "a step holds few values");

    let mut values: BTreeMap<&str, Held<'_>> = BTreeMap::new();

    for (wire, value) in members {
        let held = held_of(value)
            .ok_or_else(|| Error::Input(format!("the step value {wire} has no kind")))?;

        let _replaced = values.insert(wire.as_str(), held);
    }

    assert_eq!(values.len(), members.len(), "each value is held once");
    assert!(!values.contains_key(WIRE_BODY), "a body is no value");

    Ok(values)
}
