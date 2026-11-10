#[derive(Clone, Copy, Debug)]
pub struct PageBulk;

impl PageBulk {
    pub const AUTHOR_ID: crate::Field<Self, crate::Text> = crate::Field::new("authorId");
    pub const BODY: crate::Field<Self, crate::Object<super::BodyBulk>> = crate::Field::new("body");
    pub const CREATED_AT: crate::Field<Self, crate::Text> = crate::Field::new("createdAt");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::AbstractPageLinks>> =
        crate::Field::new("_links");

    pub const OWNER_ID: crate::Field<Self, crate::Text> = crate::Field::new("ownerId");
    pub const OWNER_ID_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastOwnerId");
    pub const PARENT_ID: crate::Field<Self, crate::Text> = crate::Field::new("parentId");
    pub const PARENT_TYPE: crate::Field<Self, crate::Text> = crate::Field::new("parentType");
    pub const POSITION: crate::Field<Self, crate::Integer> = crate::Field::new("position");
    pub const SPACE_ID: crate::Field<Self, crate::Text> = crate::Field::new("spaceId");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const SUBTYPE: crate::Field<Self, crate::Text> = crate::Field::new("subtype");
    pub const TITLE: crate::Field<Self, crate::Text> = crate::Field::new("title");

    pub const VERSION: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("version");
}
