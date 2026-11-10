#[derive(Clone, Copy, Debug)]
pub struct ContentHistoryExpandable;

impl ContentHistoryExpandable {
    pub const CONTRIBUTORS: crate::Field<Self, crate::Text> = crate::Field::new("contributors");
    pub const OWNED_BY: crate::Field<Self, crate::Text> = crate::Field::new("ownedBy");
    pub const OWNED_BY_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastOwnedBy");
    pub const UPDATED_LAST: crate::Field<Self, crate::Text> = crate::Field::new("lastUpdated");
    pub const VERSION_NEXT: crate::Field<Self, crate::Text> = crate::Field::new("nextVersion");

    pub const VERSION_PREVIOUS: crate::Field<Self, crate::Text> =
        crate::Field::new("previousVersion");
}
