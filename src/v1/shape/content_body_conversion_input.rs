#[derive(Clone, Copy, Debug)]
pub struct ContentBodyConversionInput;

impl ContentBodyConversionInput {
    pub const ALLOW_CACHE: crate::Field<Self, crate::Boolean> = crate::Field::new("allowCache");

    pub const BODY: crate::Field<Self, crate::Object<super::ContentBodyCreate>> =
        crate::Field::new("body");

    pub const CONTENT_ID_CONTEXT: crate::Field<Self, crate::Text> =
        crate::Field::new("contentIdContext");

    pub const EMBEDDED_CONTENT_RENDER: crate::Field<Self, crate::Text> =
        crate::Field::new("embeddedContentRender");

    pub const EXPAND: crate::Field<Self, crate::List<crate::Text>> = crate::Field::new("expand");

    pub const SPACE_KEY_CONTEXT: crate::Field<Self, crate::Text> =
        crate::Field::new("spaceKeyContext");

    pub const TO_VALUE: crate::Field<Self, crate::Text> = crate::Field::new("to");
}
