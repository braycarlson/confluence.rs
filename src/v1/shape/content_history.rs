#[derive(Clone, Copy, Debug)]
pub struct ContentHistory;

impl ContentHistory {
    pub const CONTRIBUTORS: crate::Field<Self, crate::Object<super::ContentHistoryContributors>> =
        crate::Field::new("contributors");

    pub const CREATED_BY: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("createdBy");

    pub const CREATED_DATE: crate::Field<Self, crate::Text> = crate::Field::new("createdDate");

    pub const EXPANDABLE: crate::Field<Self, crate::Object<super::ContentHistoryExpandable>> =
        crate::Field::new("_expandable");

    pub const LATEST: crate::Field<Self, crate::Boolean> = crate::Field::new("latest");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const OWNED_BY: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("ownedBy");

    pub const OWNED_BY_LAST: crate::Field<Self, crate::Object<super::User>> =
        crate::Field::new("lastOwnedBy");

    pub const UPDATED_LAST: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("lastUpdated");

    pub const VERSION_NEXT: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("nextVersion");

    pub const VERSION_PREVIOUS: crate::Field<Self, crate::Object<super::Version>> =
        crate::Field::new("previousVersion");
}
