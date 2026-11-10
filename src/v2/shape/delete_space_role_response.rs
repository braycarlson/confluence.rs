#[derive(Clone, Copy, Debug)]
pub struct DeleteSpaceRoleResponse;

impl DeleteSpaceRoleResponse {
    pub const TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("taskId");
}
