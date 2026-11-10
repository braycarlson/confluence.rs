#[derive(Clone, Copy, Debug)]
pub struct BulkTransitionTaskResponse;

impl BulkTransitionTaskResponse {
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const STATUS_URL: crate::Field<Self, crate::Text> = crate::Field::new("statusUrl");
    pub const TASK_ID: crate::Field<Self, crate::Text> = crate::Field::new("taskId");
}
