#[derive(Clone, Copy, Debug)]
pub struct GetTasksReply;

impl GetTasksReply {
    pub const LINKS: crate::Field<Self, crate::Object<super::MultiEntityLinks>> =
        crate::Field::new("_links");

    pub const RESULTS: crate::Field<Self, crate::List<crate::Object<super::Task>>> =
        crate::Field::new("results");
}
