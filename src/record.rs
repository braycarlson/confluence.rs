use core::marker::PhantomData;
use core::ptr;

use serde_json::Value;

use crate::field::Field;
use crate::kind::Kind;

#[derive(Clone, Copy, Debug)]
pub struct Record<'value, Schema> {
    schema: PhantomData<Schema>,
    value: &'value Value,
}

impl<'value, Schema> Record<'value, Schema> {
    #[must_use]
    pub fn member<Member>(self, field: Field<Schema, Member>) -> Option<Member::Output<'value>>
    where
        Member: Kind,
    {
        let name = field.name();

        assert!(self.value.is_object(), "a record wraps an object");
        assert!(!name.is_empty(), "a field names its member");

        Member::read(self.value.get(name)?)
    }

    #[must_use]
    pub(crate) fn of(value: &'value Value) -> Option<Self> {
        if !value.is_object() {
            return None;
        }

        let record = Self { schema: PhantomData, value };

        assert!(record.value.is_object(), "a record wraps an object");
        assert!(ptr::eq(record.value, value), "a record borrows what it was given");

        Some(record)
    }

    #[must_use]
    pub fn value(self) -> &'value Value {
        assert!(self.value.is_object(), "a record wraps an object");
        assert!(self.value.as_object().is_some(), "a record reads back as an object");

        self.value
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Record;
    use crate::field::Field;
    use crate::kind::{Integer, Object, Text};

    const AUTHOR: Field<Comment, Object<Author>> = Field::new("author");
    const ID: Field<Comment, Text> = Field::new("id");
    const NAME: Field<Author, Text> = Field::new("displayName");
    const REPLY: Field<Comment, Object<Comment>> = Field::new("reply");
    const WRONG: Field<Comment, Integer> = Field::new("id");

    #[derive(Clone, Copy, Debug)]
    struct Author;

    #[derive(Clone, Copy, Debug)]
    struct Comment;

    #[test]
    fn a_member_of_the_wrong_kind_or_missing_reads_as_absent() {
        let value = json!({"id": "10"});
        let comment = Record::<Comment>::of(&value).expect("an object is a record");

        assert_eq!(comment.member(WRONG), None, "the spec says integer and the wire says text");
        assert!(comment.member(AUTHOR).is_none(), "a missing member is absent");
    }

    #[test]
    fn a_member_reads_across_records_and_through_a_recursive_schema() {
        let value = json!({
            "author": {"displayName": "Ada"},
            "id": "10",
            "reply": {"id": "11"},
        });

        let comment = Record::<Comment>::of(&value).expect("an object is a record");
        let author = comment.member(AUTHOR).expect("the author is present");
        let reply = comment.member(REPLY).expect("the reply is present");

        assert_eq!(comment.member(ID), Some("10"), "a text member");
        assert_eq!(author.member(NAME), Some("Ada"), "a member of a nested record");
        assert_eq!(reply.member(ID), Some("11"), "a member of a record of the same schema");
    }

    #[test]
    fn a_record_keeps_every_member_the_schema_never_named() {
        let value = json!({"customfield_10001": "kept", "id": "10"});
        let comment = Record::<Comment>::of(&value).expect("an object is a record");

        assert_eq!(comment.value()["customfield_10001"], "kept", "the raw value stays reachable");
    }

    #[test]
    fn only_an_object_is_a_record() {
        assert!(Record::<Comment>::of(&json!([1_i64])).is_none(), "an array is no record");
        assert!(Record::<Comment>::of(&json!(null)).is_none(), "a null is no record");
    }
}
