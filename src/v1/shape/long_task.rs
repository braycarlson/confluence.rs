#[derive(Clone, Copy, Debug)]
pub struct LongTask;

impl LongTask {
    pub const ARI: crate::Field<Self, crate::Text> = crate::Field::new("ari");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");

    pub const LINKS: crate::Field<Self, crate::Object<super::LongTaskLinks>> =
        crate::Field::new("links");
}
