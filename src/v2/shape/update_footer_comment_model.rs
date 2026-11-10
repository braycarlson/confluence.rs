#[derive(Clone, Copy, Debug)]
pub struct UpdateFooterCommentModel;

impl UpdateFooterCommentModel {
    pub const BODY: crate::Field<Self, crate::Any> = crate::Field::new("body");

    pub const VERSION: crate::Field<Self, crate::Object<super::UpdateFooterCommentModelVersion>> =
        crate::Field::new("version");
}
