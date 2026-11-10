use core::marker::PhantomData;

use serde_json::map::Iter;
use serde_json::{Map, Value};

use crate::kind::Kind;
use crate::size;

#[derive(Clone, Debug)]
pub struct Entries<'value, Member> {
    member: PhantomData<Member>,
    values: Iter<'value>,
}

impl<'value, Member> Entries<'value, Member> {
    #[must_use]
    pub(crate) fn of(values: &'value Map<String, Value>) -> Self {
        let entries = Self { member: PhantomData, values: values.iter() };

        assert_eq!(entries.values.len(), values.len(), "a fresh walk holds every entry");
        assert!(entries.values.len() <= values.len(), "a walk holds no more than the object");

        entries
    }

    #[must_use]
    pub(crate) fn remaining_count(&self) -> u64 {
        let remaining = self.values.len();
        let count = size::of_offset(remaining);

        assert_eq!(size::offset(count), remaining, "a count is what remains");
        assert_eq!(count == 0, remaining == 0, "a count is zero exactly when done");

        count
    }
}

impl<'value, Member> Iterator for Entries<'value, Member>
where
    Member: Kind,
{
    type Item = (&'value str, Member::Output<'value>);

    fn next(&mut self) -> Option<Self::Item> {
        let remaining_before = self.remaining_count();

        for _entry_index in 0..remaining_before {
            let Some((key, value)) = self.values.next() else {
                break;
            };

            let Some(member) = Member::read(value) else {
                continue;
            };

            assert!(self.remaining_count() < remaining_before, "a walk moves forward");

            return Some((key.as_str(), member));
        }

        assert_eq!(self.remaining_count(), 0, "a walk that found nothing has nothing left");

        None
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Entries;
    use crate::kind::{Integer, Text};

    #[test]
    fn a_walk_hands_back_each_entry_of_its_kind() {
        let value = json!({"a": "one", "b": 2_i64, "c": "three"});
        let entries = Entries::<Text>::of(value.as_object().expect("an object"));
        let found: Vec<(&str, &str)> = entries.collect();

        assert_eq!(found, [("a", "one"), ("c", "three")], "the integer is passed over");
        assert_eq!(found.len(), 2, "and no more");
    }

    #[test]
    fn a_walk_with_no_entry_of_its_kind_ends_empty() {
        let value = json!({"a": "one"});
        let mut entries = Entries::<Integer>::of(value.as_object().expect("an object"));
        let found = entries.next();

        assert_eq!(found, None, "nothing reads as an integer");
        assert_eq!(entries.remaining_count(), 0, "the walk is spent");
    }
}
