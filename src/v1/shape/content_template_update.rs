#[derive(Clone, Copy, Debug)]
pub struct ContentTemplateUpdate;

impl ContentTemplateUpdate {
    pub const BODY: crate::Field<Self, crate::Object<super::ContentTemplateBodyCreate>> =
        crate::Field::new("body");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");

    pub const LABELS: crate::Field<Self, crate::List<crate::Object<super::Label>>> =
        crate::Field::new("labels");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SPACE: crate::Field<Self, crate::Object<super::ContentTemplateUpdateSpace>> =
        crate::Field::new("space");

    pub const TEMPLATE_ID: crate::Field<Self, crate::Text> = crate::Field::new("templateId");
    pub const TEMPLATE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("templateType");
}
