use alloc::{collections::VecDeque, sync::Arc};

use crate::sync::UniProcSafeCell;

use super::task::ProcessControlBlock;

pub struct ProcScheduler {
    rdy_que: VecDeque<Arc<ProcessControlBlock>>,
}

/// simple FIFO scheduler
impl ProcScheduler {
    pub fn new() -> Self {
        Self {
            rdy_que: VecDeque::new(),
        }
    }

    pub fn add(&mut self, task: Arc<ProcessControlBlock>) {
        self.rdy_que.push_back(task)
    }

    pub fn fetch(&mut self) -> Option<Arc<ProcessControlBlock>> {
        self.rdy_que.pop_front()
    }
}

lazy_static! {
    pub static ref PROC_SCHEDULER: UniProcSafeCell<ProcScheduler> =
        UniProcSafeCell::new(ProcScheduler::new());
}

pub fn add_task(task: Arc<ProcessControlBlock>) {
    PROC_SCHEDULER.borrow_mut().add(task)
}

pub fn fetch_task() -> Option<alloc::sync::Arc<ProcessControlBlock>> {
    PROC_SCHEDULER.borrow_mut().fetch()
}
