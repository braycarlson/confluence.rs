use core::marker::PhantomData;

use serde_json::Value;

use crate::items::Items;
use crate::mapping::Mapping;
use crate::record::Record;

#[derive(Clone, Copy, Debug)]
pub struct Any;

#[derive(Clone, Copy, Debug)]
pub struct Binary;

#[derive(Clone, Copy, Debug)]
pub struct Boolean;

#[derive(Clone, Copy, Debug)]
pub struct Integer;

#[derive(Clone, Copy, Debug)]
pub struct List<Item>(PhantomData<Item>);

#[derive(Clone, Copy, Debug)]
pub struct Map<Member>(PhantomData<Member>);

#[derive(Clone, Copy, Debug)]
pub struct Nothing;

#[derive(Clone, Copy, Debug)]
pub struct Number;

#[derive(Clone, Copy, Debug)]
pub struct Object<Schema>(PhantomData<Schema>);

#[derive(Clone, Copy, Debug)]
pub struct Text;

pub trait Kind {
    const IS_BODY_IGNORED: bool;

    type Output<'value>;

    fn read(value: &Value) -> Option<Self::Output<'_>>;
}

impl Kind for Any {
    const IS_BODY_IGNORED: bool = false;

    type Output<'value> = &'value Value;

    fn read(value: &Value) -> Option<Self::Output<'_>> {
        Some(value)
    }
}

impl Kind for Boolean {
    const IS_BODY_IGNORED: bool = false;

    type Output<'value> = bool;

    fn read(value: &Value) -> Option<Self::Output<'_>> {
        value.as_bool()
    }
}

impl Kind for Integer {
    const IS_BODY_IGNORED: bool = false;

    type Output<'value> = i64;

    fn read(value: &Value) -> Option<Self::Output<'_>> {
        value.as_i64()
    }
}

impl<Item> Kind for List<Item>
where
    Item: Kind,
{
    const IS_BODY_IGNORED: bool = false;

    type Output<'value> = Items<'value, Item>;

    fn read(value: &Value) -> Option<Self::Output<'_>> {
        Some(Items::of(value.as_array()?))
    }
}

impl<Member> Kind for Map<Member>
where
    Member: Kind,
{
    const IS_BODY_IGNORED: bool = false;

    type Output<'value> = Mapping<'value, Member>;

    fn read(value: &Value) -> Option<Self::Output<'_>> {
        Some(Mapping::of(value.as_object()?))
    }
}

impl Kind for Nothing {
    const IS_BODY_IGNORED: bool = true;

    type Output<'value> = ();

    fn read(_value: &Value) -> Option<Self::Output<'_>> {
        Some(())
    }
}

impl Kind for Number {
    const IS_BODY_IGNORED: bool = false;

    type Output<'value> = f64;

    fn read(value: &Value) -> Option<Self::Output<'_>> {
        value.as_f64()
    }
}

impl<Schema> Kind for Object<Schema> {
    const IS_BODY_IGNORED: bool = false;

    type Output<'value> = Record<'value, Schema>;

    fn read(value: &Value) -> Option<Self::Output<'_>> {
        Record::of(value)
    }
}

impl Kind for Text {
    const IS_BODY_IGNORED: bool = false;

    type Output<'value> = &'value str;

    fn read(value: &Value) -> Option<Self::Output<'_>> {
        value.as_str()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{Any, Boolean, Integer, Kind as _, List, Map, Nothing, Number, Object, Text};

    #[test]
    fn a_value_of_the_wrong_shape_reads_as_absent_rather_than_failing() {
        let text = json!("words");
        let integer = json!(12_i64);

        assert!(Boolean::read(&text).is_none(), "a text is no boolean");
        assert!(Integer::read(&text).is_none(), "a text is no integer");
        assert!(Number::read(&text).is_none(), "a text is no number");
        assert!(Text::read(&integer).is_none(), "an integer is no text");
        assert!(<List<Text>>::read(&integer).is_none(), "an integer is no list");
        assert!(<Map<Text>>::read(&integer).is_none(), "an integer is no map");
        assert!(<Object<Text>>::read(&integer).is_none(), "an integer is no record");
    }

    #[test]
    fn each_kind_reads_the_shape_it_names() {
        assert_eq!(Boolean::read(&json!(true)), Some(true), "a boolean");
        assert_eq!(Integer::read(&json!(-3_i64)), Some(-3), "an integer");
        assert_eq!(Number::read(&json!(1.5_f64)), Some(1.5_f64), "a number");
        assert_eq!(Text::read(&json!("word")), Some("word"), "a text");
        assert_eq!(Any::read(&json!(null)), Some(&json!(null)), "anything at all");
    }

    #[test]
    fn nothing_reads_from_anything() {
        assert_eq!(Nothing::read(&json!({"ignored": true})), Some(()), "an object");
        assert_eq!(Nothing::read(&json!(null)), Some(()), "a null");
    }
}
