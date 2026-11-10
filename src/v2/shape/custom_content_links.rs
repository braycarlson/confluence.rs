#[derive(Clone, Copy, Debug)]
pub struct CustomContentLinks;

impl CustomContentLinks {
    pub const WEBUI: crate::Field<Self, crate::Text> = crate::Field::new("webui");
}
