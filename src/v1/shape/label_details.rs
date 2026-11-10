#[derive(Clone, Copy, Debug)]
pub struct LabelDetails;

impl LabelDetails {
    pub const ASSOCIATED_CONTENTS: crate::Field<
        Self,
        crate::Object<super::LabeledContentPageResponse>,
    > = crate::Field::new("associatedContents");

    pub const LABEL: crate::Field<Self, crate::Object<super::Label>> = crate::Field::new("label");
}
