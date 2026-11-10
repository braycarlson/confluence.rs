#[derive(Clone, Copy, Debug)]
pub struct GetSpaceRolesByIDReply;

impl GetSpaceRolesByIDReply {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::GetSpaceRolesByIDReplyLinks>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");

    pub const SPACE_PERMISSIONS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("spacePermissions");

    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
