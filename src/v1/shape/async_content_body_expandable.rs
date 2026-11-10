#[derive(Clone, Copy, Debug)]
pub struct AsyncContentBodyExpandable;

impl AsyncContentBodyExpandable {
    pub const CONTENT: crate::Field<Self, crate::Text> = crate::Field::new("content");

    pub const EMBEDDED_CONTENT: crate::Field<Self, crate::Text> =
        crate::Field::new("embeddedContent");

    pub const MEDIA_TOKEN: crate::Field<Self, crate::Text> = crate::Field::new("mediaToken");
    pub const WEBRESOURCE: crate::Field<Self, crate::Text> = crate::Field::new("webresource");
}
