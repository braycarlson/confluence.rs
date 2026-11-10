#[derive(Clone, Copy, Debug)]
pub struct CreateInlineCommentModelInlineCommentProperties;

impl CreateInlineCommentModelInlineCommentProperties {
    pub const TEXT_SELECTION: crate::Field<Self, crate::Text> = crate::Field::new("textSelection");

    pub const TEXT_SELECTION_MATCH_COUNT: crate::Field<Self, crate::Integer> =
        crate::Field::new("textSelectionMatchCount");

    pub const TEXT_SELECTION_MATCH_INDEX: crate::Field<Self, crate::Integer> =
        crate::Field::new("textSelectionMatchIndex");
}
