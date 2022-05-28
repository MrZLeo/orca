mod context;
pub mod kernel_stack;
pub mod pid;
pub mod processor;
pub mod scheduler;
mod switch;
#[allow(clippy::module_inception)]
pub mod task;

use crate::fs::inode::open_file;
use crate::fs::inode::OpenFlags;
use crate::loader::app_from_name;
use alloc::sync::Arc;

pub use context::TaskContext;
use lazy_static::*;
pub use switch::__switch;
use task::{ProcessControlBlock, TaskStatus};

use self::processor::cur_task;
pub use self::processor::run;
use self::processor::schedule;
use self::processor::take_cur_task;
use self::scheduler::add_task;

lazy_static! {
    pub static ref INITPROC: Arc<ProcessControlBlock> = Arc::new({
        let inode = open_file("initproc", OpenFlags::RDONLY).unwrap();
        let data = inode.read_all();
        ProcessControlBlock::new(data.as_slice())
    });
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

pub fn exit_cur_and_run_next(exit_code: i32) {
    let task = take_cur_task().unwrap();
    let mut inner = task.borrow_mut();

    inner.status = TaskStatus::Zombie;
    inner.exit_code = exit_code;

    let mut initproc = INITPROC.borrow_mut();
    for child in inner.children.iter() {
        child.borrow_mut().parent = Some(Arc::downgrade(&INITPROC));
        initproc.children.push(child.clone());
    }
    drop(initproc);

    inner.children.clear();
    inner.memory_set.recycle_pages();
    drop(inner);
    drop(task);

    let mut _unused = TaskContext::from_zero();
    schedule(&mut _unused as *mut TaskContext);
}
