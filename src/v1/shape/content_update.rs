#[derive(Clone, Copy, Debug)]
pub struct ContentUpdate;

impl ContentUpdate {
    pub const ANCESTORS: crate::Field<
        Self,
        crate::List<crate::Object<super::ContentUpdateAncestors>>,
    > = crate::Field::new("ancestors");

    pub const BODY: crate::Field<Self, crate::Object<super::ContentUpdateBody>> =
        crate::Field::new("body");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");

    pub const VERSION: crate::Field<Self, crate::Object<super::ContentUpdateVersion>> =
        crate::Field::new("version");
}
