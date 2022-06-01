use super::__switch;
use super::scheduler::fetch_task;
use super::task::TaskStatus;
use super::{task::ProcessControlBlock, TaskContext};
use crate::sync::UniProcSafeCell;
use crate::trap::TrapContext;
use alloc::sync::Arc;

/// # Processor
/// Processor is the abstraction of one HART(a special concept in RISC-V)
/// For spercific, it means a running processor
///
/// We will shift a running process into this strcture and manage it.
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

    /// use `take()` of Option to get ownership
    pub fn take_cur(&mut self) -> Option<Arc<ProcessControlBlock>> {
        self.cur.take()
    }

    /// current running process of this processor
    pub fn cur(&self) -> Option<Arc<ProcessControlBlock>> {
        self.cur.as_ref().map(Arc::clone)
    }

    /// get idel process's TaskContext
    pub fn idle_task_cxt_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cxt as *mut TaskContext
    }
}

lazy_static! {
    pub static ref PROCESSOR: UniProcSafeCell<Processor> = UniProcSafeCell::new(Processor::new());
}

pub fn take_cur_task() -> Option<Arc<ProcessControlBlock>> {
    PROCESSOR.borrow_mut().take_cur()
}

pub fn cur_task() -> Option<Arc<ProcessControlBlock>> {
    PROCESSOR.borrow_mut().cur()
}

pub fn cur_user_token() -> usize {
    let task = cur_task().unwrap();
    let x = task.borrow_mut().user_token();
    x
}

pub fn cur_trap_cxt() -> &'static mut TrapContext {
    cur_task().unwrap().borrow_mut().trap_cxt()
}

/// # processor::run
/// start running processor
/// > warn: endless loop
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

            // use switch.S, change idle -> next
            unsafe {
                __switch(idle_ptr, next_ptr);
            }
        }
    }
}

/// # processor::schedule
/// change different task to run
/// > warn: in detail, we will shift to *idle_task* first, and *idel_task* will switch to other available task
pub fn schedule(cur_task: *mut TaskContext) {
    let mut processor = PROCESSOR.borrow_mut();
    let idle_ptr = processor.idle_task_cxt_ptr();
    drop(processor);
    unsafe {
        __switch(cur_task, idle_ptr);
    }
}
