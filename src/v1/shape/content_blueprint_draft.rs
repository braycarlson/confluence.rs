#[derive(Clone, Copy, Debug)]
pub struct ContentBlueprintDraft;

impl ContentBlueprintDraft {
    pub const ANCESTORS: crate::Field<
        Self,
        crate::List<crate::Object<super::ContentBlueprintDraftAncestors>>,
    > = crate::Field::new("ancestors");

    pub const SPACE: crate::Field<Self, crate::Object<super::ContentBlueprintDraftSpace>> =
        crate::Field::new("space");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VERSION: crate::Field<Self, crate::Object<super::ContentBlueprintDraftVersion>> =
        crate::Field::new("version");
}
