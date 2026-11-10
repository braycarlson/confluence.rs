#[derive(Clone, Copy, Debug)]
pub struct InlineCommentProperties;

impl InlineCommentProperties {
    pub const INLINE_MARKER_REFERENCE: crate::Field<Self, crate::Text> =
        crate::Field::new("inlineMarkerRef");

    pub const INLINE_ORIGINAL_SELECTION: crate::Field<Self, crate::Text> =
        crate::Field::new("inlineOriginalSelection");
}
