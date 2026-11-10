#[derive(Clone, Copy, Debug)]
pub struct Template;

impl Template {
    pub const CATEGORIES: crate::Field<Self, crate::List<crate::Object<super::TemplateCategory>>> =
        crate::Field::new("categories");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const DISPLAY_METADATA: crate::Field<Self, crate::Object<super::TemplateDisplayMetadata>> =
        crate::Field::new("displayMetadata");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const PARAMETERS: crate::Field<
        Self,
        crate::List<crate::Object<super::CreateRuleFromTemplateParameter>>,
    > = crate::Field::new("parameters");
}
