#[derive(Clone, Copy, Debug)]
pub struct ContentMetadataCurrentuser;

impl ContentMetadataCurrentuser {
    pub const EXPANDABLE: crate::Field<
        Self,
        crate::Object<super::ContentMetadataCurrentuserExpandable>,
    > = crate::Field::new("_expandable");

    pub const FAVOURITED: crate::Field<
        Self,
        crate::Object<super::ContentMetadataCurrentuserFavourited>,
    > = crate::Field::new("favourited");

    pub const LASTCONTRIBUTED: crate::Field<
        Self,
        crate::Object<super::ContentMetadataCurrentuserLastcontributed>,
    > = crate::Field::new("lastcontributed");

    pub const LASTMODIFIED: crate::Field<
        Self,
        crate::Object<super::ContentMetadataCurrentuserLastmodified>,
    > = crate::Field::new("lastmodified");

    pub const SCHEDULED: crate::Field<Self, crate::Any> = crate::Field::new("scheduled");

    pub const VIEWED: crate::Field<Self, crate::Object<super::ContentMetadataCurrentuserViewed>> =
        crate::Field::new("viewed");
}
