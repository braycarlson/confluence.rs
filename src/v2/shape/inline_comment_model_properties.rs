#[derive(Clone, Copy, Debug)]
pub struct InlineCommentModelProperties;

impl InlineCommentModelProperties {
    pub const INLINE_MARKER_REFERENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("inlineMarkerRef");

    pub const INLINE_ORIGINAL_SELECTION: crate::Field<Self, crate::Text> =
        crate::Field::new("inlineOriginalSelection");

    pub const LINKS: crate::Field<Self, crate::Object<super::OptionalFieldLinks>> =
        crate::Field::new("_links");

    pub const META: crate::Field<Self, crate::Object<super::OptionalFieldMeta>> =
        crate::Field::new("meta");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::ContentProperty>>> =
        crate::Field::new("results");
}
