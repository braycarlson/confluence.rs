#[derive(Clone, Copy, Debug)]
pub struct UpdateInlineCommentModel;

impl UpdateInlineCommentModel {
    pub const BODY: crate::Field<Self, crate::Any> = crate::Field::new("body");
    pub const RESOLVED: crate::Field<Self, crate::Boolean> = crate::Field::new("resolved");

    pub const VERSION: crate::Field<Self, crate::Object<super::UpdateInlineCommentModelVersion>> =
        crate::Field::new("version");
}
