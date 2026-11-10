#[derive(Clone, Copy, Debug)]
pub struct SpaceLinks;

impl SpaceLinks {
    pub const WEBUI: crate::Field<Self, crate::Text> = crate::Field::new("webui");
}
