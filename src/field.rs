use core::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct Field<Schema, Member> {
    member: PhantomData<Member>,
    name: &'static str,
    schema: PhantomData<Schema>,
}

impl<Schema, Member> Field<Schema, Member> {
    #[must_use]
    pub const fn name(self) -> &'static str {
        assert!(!self.name.is_empty(), "a field names its member");
        assert!(self.name.len() <= 512, "a member name is a short text");

        self.name
    }

    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        assert!(!name.is_empty(), "a field names its member");

        let field = Self { member: PhantomData, name, schema: PhantomData };

        assert!(field.name.len() == name.len(), "a field holds the name it was given");
        assert!(name.len() <= 512, "a member name is a short text");

        field
    }
}

#[cfg(test)]
mod tests {
    use super::Field;
    use crate::kind::Text;

    const NAME: Field<Schema, Text> = Field::new("displayName");

    #[derive(Clone, Copy, Debug)]
    struct Schema;

    #[test]
    fn a_field_keeps_the_name_it_was_built_with() {
        assert_eq!(NAME.name(), "displayName", "the wire name survives");
        assert!(!NAME.name().is_empty(), "a built field is always named");
    }

    #[test]
    #[should_panic(expected = "a field names its member")]
    fn a_field_with_no_name_is_a_programmer_error() {
        let field = <Field<Schema, Text>>::new("");

        assert!(field.name().is_empty(), "the panic above is the point of this test");
    }
}
