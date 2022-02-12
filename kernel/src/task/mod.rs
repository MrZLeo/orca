mod context;
mod switch;
#[allow(clippy::module_inception)]
mod task;

use crate::loader::get_app_data;
use crate::loader::num_app;
use crate::sync::UniProcSafeCell;
use crate::trap::TrapContext;
use alloc::vec::Vec;
pub use context::TaskContext;
use lazy_static::*;
pub use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub struct TaskScheduler {
    num_app: usize,
    inner: UniProcSafeCell<TaskSchedulerInner>,
}

struct TaskSchedulerInner {
    tasks: Vec<TaskControlBlock>,
    cur_task: usize, // current task's index
}

impl TaskScheduler {
    /// `cur` == usize::MAX means running first task of os
    fn run(&self, next: usize) {
        let mut inner = self.inner.borrow_mut();
        let cur_task = inner.cur_task;

        inner.tasks[next].status = TaskStatus::Running;
        inner.cur_task = next;

        let next_ptr = &inner.tasks[next].cxt as *const TaskContext;
        let cur_ptr = &mut inner.tasks[cur_task].cxt as *mut TaskContext;
        drop(inner);
        unsafe {
            __switch(cur_ptr, next_ptr);
        }
    }

    fn set_cur_status(&self, status: TaskStatus) {
        let mut inner = self.inner.borrow_mut();
        let cur = inner.cur_task;
        inner.tasks[cur].status = status;
    }

    fn cur_suspend(&self) {
        self.set_cur_status(TaskStatus::Ready);
    }

    fn cur_exit(&self) {
        self.set_cur_status(TaskStatus::Exited);
    }

    fn next_task(&self) -> Option<usize> {
        let inner = self.inner.borrow_mut();
        let cur = inner.cur_task;
        (cur + 1..cur + 1 + self.num_app)
            .map(|id| id % self.num_app)
            .find(|&id| inner.tasks[id].status == TaskStatus::Ready)
    }

    fn run_next(&self) {
        if let Some(next) = self.next_task() {
            self.run(next)
        } else {
            panic!("All tasks finished");
        }
    }

    fn start(&self) {
        let mut inner = self.inner.borrow_mut();
        let task0 = &mut inner.tasks[0];
        task0.status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.cxt as *const TaskContext;
        debug!("shift to {} at 0x{:x}", 0, inner.tasks[0].cxt.sp() as usize);
        drop(inner);
        let mut _unused = TaskContext::from_zero();
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

    fn cur_token(&self) -> usize {
        let inner = self.inner.borrow_mut();
        let cur = inner.cur_task;
        inner.tasks[cur].user_token()
    }

    fn cur_trap_cxt(&self) -> &mut TrapContext {
        let mut inner = self.inner.borrow_mut();
        let cur = inner.cur_task;
        inner.tasks[cur].trap_cxt()
    }
}

lazy_static! {
    pub static ref TASK_SCHEDULER: TaskScheduler = {
        info!("init TASK_SCHEDULER");
        let num_app = num_app();
        info!("num_app = {}", num_app);
        let mut tasks: Vec<TaskControlBlock> = Vec::new();
        for i in 0..num_app {
            tasks.push(TaskControlBlock::new(get_app_data(i), i));
        }

        TaskScheduler {
            num_app,
            inner: UniProcSafeCell::new(TaskSchedulerInner { tasks, cur_task: 0 }),
        }
    };
}

/// start running apps
pub fn start() {
    TASK_SCHEDULER.start();
}

/// stop cur app (make it sleep and change cpu to other app)
pub fn cur_suspend() {
    TASK_SCHEDULER.cur_suspend();
}

/// run next app
pub fn run_next() {
    TASK_SCHEDULER.run_next();
}

/// cur app finished or cause error and be killed
pub fn cur_exit() {
    TASK_SCHEDULER.cur_exit();
}

pub fn cur_user_token() -> usize {
    TASK_SCHEDULER.cur_token()
}

pub fn cur_trap_cxt() -> &'static mut TrapContext {
    TASK_SCHEDULER.cur_trap_cxt()
}
