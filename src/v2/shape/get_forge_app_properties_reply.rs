#[derive(Clone, Copy, Debug)]
pub struct GetForgeAppPropertiesReply;

impl GetForgeAppPropertiesReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<
        Self,
        crate::List<crate::Object<super::GetForgeAppPropertiesReplyResults>>,
    > = crate::Field::new("results");
}
