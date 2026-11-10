#[derive(Clone, Copy, Debug)]
pub struct ChangedValue;

impl ChangedValue {
    pub const HIDDEN_VALUE_NEW: crate::Field<Self, crate::Text> =
        crate::Field::new("hiddenNewValue");

    pub const HIDDEN_VALUE_OLD: crate::Field<Self, crate::Text> =
        crate::Field::new("hiddenOldValue");

    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const VALUE_NEW: crate::Field<Self, crate::Text> = crate::Field::new("newValue");
    pub const VALUE_OLD: crate::Field<Self, crate::Text> = crate::Field::new("oldValue");
}
