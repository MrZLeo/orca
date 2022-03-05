mod context;
pub mod kernel_stack;
pub mod pid;
pub mod processor;
pub mod scheduler;
mod switch;
#[allow(clippy::module_inception)]
pub mod task;

use crate::loader::app_from_name;
use crate::loader::get_app_data;
use crate::loader::num_app;
use crate::sync::UniProcSafeCell;
use crate::trap::TrapContext;
use alloc::sync::Arc;
use alloc::vec::Vec;
pub use context::TaskContext;
use lazy_static::*;
pub use switch::__switch;
use task::{ProcessControlBlock, TaskStatus};

use self::processor::cur_task;
use self::processor::schedule;
use self::scheduler::add_task;

lazy_static! {
    pub static ref INITPROC: Arc<ProcessControlBlock> =
        { Arc::new(ProcessControlBlock::new(app_from_name("initproc").unwrap())) };
}

pub fn add_initproc() {
    add_task(INITPROC.clone());
}

pub fn suspend_cur_and_run_next() {
    let task = cur_task().unwrap();

    let mut task_inner = task.borrow_mut();
    let task_ptr = &mut task_inner.cxt as *mut TaskContext;

    task_inner.status = TaskStatus::Ready;
    drop(task_inner);

    add_task(task);
    schedule(task_ptr);
}
