#[derive(Clone, Copy, Debug)]
pub struct ContentPropertyUpdate;

impl ContentPropertyUpdate {
    pub const VALUE: crate::Field<Self, crate::Any> = crate::Field::new("value");

    pub const VERSION: crate::Field<Self, crate::Object<super::ContentPropertyUpdateVersion>> =
        crate::Field::new("version");
}
