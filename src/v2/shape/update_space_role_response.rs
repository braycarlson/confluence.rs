#[derive(Clone, Copy, Debug)]
pub struct UpdateSpaceRoleResponse;

impl UpdateSpaceRoleResponse {
    pub const DESCRIPTION: crate::Field<Self, crate::Text> = crate::Field::new("description");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const NAME: crate::Field<Self, crate::Text> = crate::Field::new("name");
    pub const TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("taskId");
    pub const TYPE: crate::Field<Self, crate::Text> = crate::Field::new("type");
}
