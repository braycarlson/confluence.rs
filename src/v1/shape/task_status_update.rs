#[derive(Clone, Copy, Debug)]
pub struct TaskStatusUpdate;

impl TaskStatusUpdate {
    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
}
