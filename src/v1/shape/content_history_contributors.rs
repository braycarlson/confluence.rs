#[derive(Clone, Copy, Debug)]
pub struct ContentHistoryContributors;

impl ContentHistoryContributors {
    pub const PUBLISHERS: crate::Field<Self, crate::Object<super::UsersUserKeys>> =
        crate::Field::new("publishers");
}
