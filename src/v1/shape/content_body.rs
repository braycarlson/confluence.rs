#[derive(Clone, Copy, Debug)]
pub struct ContentBody;

impl ContentBody {
    pub const EMBEDDED_CONTENT: crate::Field<
        Self,
        crate::List<crate::Object<super::EmbeddedContent>>,
    > = crate::Field::new("embeddedContent");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::ContentBodyExpandable>> =
        crate::Field::new("_expandable");

    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const MEDIA_TOKEN: crate::Field<Self, crate::Object<super::ContentBodyMediaToken>> =
        crate::Field::new("mediaToken");

    pub const REPRESENTATION: crate::Field<Self, crate::Text> = crate::Field::new("representation");
    pub const VALUE: crate::Field<Self, crate::Text> = crate::Field::new("value");

    pub const WEBRESOURCE: crate::Field<Self, crate::Object<super::WebResourceDependencies>> =
        crate::Field::new("webresource");
}
