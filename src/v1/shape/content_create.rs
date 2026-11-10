#[derive(Clone, Copy, Debug)]
pub struct ContentCreate;

impl ContentCreate {
    pub const ANCESTORS: crate::Field<
        Self,
        crate::List<crate::Object<super::ContentCreateAncestors>>,
    > = crate::Field::new("ancestors");

    pub const BODY: crate::Field<Self, crate::Object<super::ContentCreateBody>> =
        crate::Field::new("body");

    pub const CONTAINER: crate::Field<Self, crate::Object<super::ContentCreateContainer>> =
        crate::Field::new("container");

    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const SPACE: crate::Field<Self, crate::Object<super::ContentCreateSpace>> =
        crate::Field::new("space");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
