use super::TaskContext;

#[derive(Clone, Copy, PartialEq)]
pub enum TaskStatus {
    /// Task that just created
    UnInit,
    /// Task is ready to run
    Ready,
    /// Task is runnning
    Running,
    /// Task that already completed or killed
    Exited,
}

#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub cxt: TaskContext,
}
