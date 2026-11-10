use core::marker::PhantomData;
use core::ptr;

use serde_json::{Map, Value};

use crate::entries::Entries;
use crate::kind::Kind;
use crate::size;

#[derive(Clone, Copy, Debug)]
pub struct Mapping<'value, Member> {
    member: PhantomData<Member>,
    values: &'value Map<String, Value>,
}

impl<'value, Member> Mapping<'value, Member>
where
    Member: Kind,
{
    #[must_use]
    pub fn count(self) -> u64 {
        let count = size::of_offset(self.values.len());

        assert_eq!(count == 0, self.values.is_empty(), "a count is zero exactly when it is empty");
        assert!(count <= size::of_offset(usize::MAX), "a count fits an offset");

        count
    }

    #[must_use]
    pub fn entries(self) -> Entries<'value, Member> {
        let count = size::of_offset(self.values.len());
        let entries = Entries::of(self.values);

        assert_eq!(entries.remaining_count(), count, "a fresh walk holds every entry");

        entries
    }

    #[must_use]
    pub fn member(self, key: &str) -> Option<Member::Output<'value>> {
        assert!(!key.is_empty(), "a key names its member");
        assert!(key.len() <= 4_096, "a key is a short text");

        Member::read(self.values.get(key)?)
    }

    #[must_use]
    pub(crate) fn of(values: &'value Map<String, Value>) -> Self {
        let mapping = Self { member: PhantomData, values };

        assert_eq!(mapping.values.len(), values.len(), "a mapping borrows the whole object");
        assert!(ptr::eq(mapping.values, values), "a mapping borrows what it was given");

        mapping
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Mapping;
    use crate::kind::{Integer, Text};

    #[test]
    fn a_mapping_reads_a_member_by_key_and_counts_what_it_holds() {
        let value = json!({"customfield_10001": "kept", "summary": "words"});
        let fields = Mapping::<Text>::of(value.as_object().expect("an object"));

        assert_eq!(fields.member("summary"), Some("words"), "a present key");
        assert_eq!(fields.member("missing"), None, "a missing key");
        assert_eq!(fields.count(), 2, "both members count");
    }

    #[test]
    fn a_member_of_the_wrong_kind_reads_as_absent() {
        let value = json!({"count": "seven"});
        let fields = Mapping::<Integer>::of(value.as_object().expect("an object"));

        assert_eq!(fields.member("count"), None, "a text is no integer");
        assert_eq!(fields.count(), 1, "the member still counts");
        assert_eq!(fields.entries().count(), 0, "and reads as no entry of this kind");
    }

    #[test]
    #[should_panic(expected = "a key names its member")]
    fn an_empty_key_is_a_programmer_error() {
        let value = json!({});
        let member = Mapping::<Text>::of(value.as_object().expect("an object")).member("");

        assert!(member.is_none(), "the panic above is the point of this test");
    }
}
