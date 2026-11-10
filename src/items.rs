use core::marker::PhantomData;
use core::slice;

use serde_json::Value;

use crate::kind::Kind;
use crate::size;

#[derive(Clone, Debug)]
pub struct Items<'value, Item> {
    item: PhantomData<Item>,
    values: slice::Iter<'value, Value>,
}

impl<'value, Item> Items<'value, Item> {
    #[must_use]
    pub(crate) fn of(values: &'value [Value]) -> Self {
        let items = Self { item: PhantomData, values: values.iter() };

        assert_eq!(items.values.len(), values.len(), "a fresh walk holds every value");
        assert_eq!(items.values.as_slice().len(), values.len(), "a walk borrows the whole slice");

        items
    }
}

impl<'value, Item> Iterator for Items<'value, Item>
where
    Item: Kind,
{
    type Item = Item::Output<'value>;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining_before = size::of_offset(self.values.len());
        let found = self.values.find_map(Item::read);
        let remaining_after = size::of_offset(self.values.len());

        assert!(remaining_after <= remaining_before, "a walk only ever moves forward");

        if found.is_some() {
            assert!(remaining_after < remaining_before, "a value handed back is a value spent");
        }

        found
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::Items;
    use crate::kind::{Integer, Text};

    #[test]
    fn a_list_hands_back_each_value_of_its_kind_in_order() {
        let value = json!(["a", "b", "c"]);
        let words: Vec<&str> = Items::<Text>::of(value.as_array().expect("an array")).collect();

        assert_eq!(words, ["a", "b", "c"], "every value in order");
        assert_eq!(words.len(), 3, "and no more");
    }

    #[test]
    fn a_value_of_the_wrong_kind_is_skipped_rather_than_failing_the_walk() {
        let value = json!([1_i64, "a", 2_i64]);
        let numbers: Vec<i64> = Items::<Integer>::of(value.as_array().expect("an array")).collect();

        assert_eq!(numbers, [1, 2], "the text in the middle is passed over");
        assert_eq!(numbers.len(), 2, "and no more");
    }

    #[test]
    fn an_empty_list_ends_at_once() {
        let mut items = Items::<Text>::of(&[]);
        let first = items.next();
        let second = items.next();

        assert_eq!(first, None, "there is nothing to hand back");
        assert_eq!(second, None, "and there stays nothing");
    }
}
