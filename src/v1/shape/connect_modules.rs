#[derive(Clone, Copy, Debug)]
pub struct ConnectModules;

impl ConnectModules {
    pub const MODULES: crate::Field<Self, crate::List<crate::Any>> = crate::Field::new("modules");
}
