#[derive(Clone, Copy, Debug)]
pub struct ContentTemplate;

impl ContentTemplate {
    pub const BODY: crate::Field<Self, crate::Object<super::ContentTemplateBody>> =
        crate::Field::new("body");

    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const EDITOR_VERSION: crate::Field<Self, crate::Text> = crate::Field::new("editorVersion");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::ContentTemplateExpandable>> =
        crate::Field::new("_expandable");

    pub const LABELS: crate::Field<Self, crate::List<crate::Object<super::Label>>> =
        crate::Field::new("labels");

    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const ORIGINAL_TEMPLATE: crate::Field<
        Self,
        crate::Object<super::ContentTemplateOriginalTemplate>,
    > = crate::Field::new("originalTemplate");

    pub const REFERENCING_BLUEPRINT: crate::Field<Self, crate::Text> =
        crate::Field::new("referencingBlueprint");

    pub const SPACE: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("space");
    pub const TEMPLATE_ID: crate::Field<Self, crate::Text> = crate::Field::new("templateId");
    pub const TEMPLATE_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("templateType");
}
