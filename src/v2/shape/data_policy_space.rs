#[derive(Clone, Copy, Debug)]
pub struct DataPolicySpace;

impl DataPolicySpace {
    pub const DATA_POLICY: crate::Field<Self, crate::Object<super::DataPolicySpaceDataPolicy>> =
        crate::Field::new("dataPolicy");

    pub const DESCRIPTION: crate::Field<Self, crate::Object<super::SpaceDescription>> =
        crate::Field::new("description");

    pub const ICON: crate::Field<Self, crate::Object<super::SpaceIcon>> = crate::Field::new("icon");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const KEY: crate::Field<Self, crate::Text> = crate::Field::new("key");

    pub const LINKS: crate::Field<Self, crate::Object<super::SpaceLinks>> =
        crate::Field::new("_links");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
}
