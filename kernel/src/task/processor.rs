use super::scheduler::fetch_task;
use super::task::TaskStatus;
use super::{__switch, switch};
use super::{task::ProcessControlBlock, TaskContext};
use crate::sync::UniProcSafeCell;
use crate::trap::TrapContext;
use alloc::sync::Arc;

pub struct Processor {
    cur: Option<Arc<ProcessControlBlock>>,
    idle_task_cxt: TaskContext,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            cur: None,
            idle_task_cxt: TaskContext::from_zero(),
        }
    }

    pub fn take_cur(&mut self) -> Option<Arc<ProcessControlBlock>> {
        self.cur.take()
    }

    pub fn cur(&self) -> Option<Arc<ProcessControlBlock>> {
        self.cur.as_ref().map(|task| Arc::clone(task))
    }

    pub fn idle_task_cxt_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cxt as *mut TaskContext
    }
}

lazy_static! {
    pub static ref PROCESSOR: UniProcSafeCell<Processor> =
        { UniProcSafeCell::new(Processor::new()) };
}

pub fn take_cur_task() -> Option<Arc<ProcessControlBlock>> {
    PROCESSOR.borrow_mut().take_cur()
}

pub fn cur_task() -> Option<Arc<ProcessControlBlock>> {
    PROCESSOR.borrow_mut().cur()
}

pub fn cur_user_token() -> usize {
    let task = cur_task().unwrap();
    task.borrow_mut().user_token()
}

pub fn cur_trap_cxt() -> &'static mut TrapContext {
    cur_task().unwrap().borrow_mut().trap_cxt()
}

pub fn run() {
    loop {
        let mut processor = PROCESSOR.borrow_mut();
        if let Some(task) = fetch_task() {
            let idle_ptr = processor.idle_task_cxt_ptr();
            let mut task_inner = task.borrow_mut();
            let next_ptr = &task_inner.cxt as *const TaskContext;
            task_inner.status = TaskStatus::Running;
            drop(task_inner);
            processor.cur = Some(task);
            drop(processor);

            unsafe {
                __switch(idle_ptr, next_ptr);
            }
        }
    }
}

pub fn schedule(cur_task: *mut TaskContext) {
    let mut processor = PROCESSOR.borrow_mut();
    let idle_ptr = processor.idle_task_cxt_ptr();
    drop(processor);
    unsafe {
        __switch(cur_task, idle_ptr);
    }
}
