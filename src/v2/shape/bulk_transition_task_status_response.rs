#[derive(Clone, Copy, Debug)]
pub struct BulkTransitionTaskStatusResponse;

impl BulkTransitionTaskStatusResponse {
    pub const ERROR_MESSAGE: crate::Field<Self, crate::Text> = crate::Field::new("errorMessage");
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("taskId");
}
