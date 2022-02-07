mod context;
mod switch;
#[allow(clippy::module_inception)]
mod task;

use crate::config::MAX_APP_NUM;
use crate::loader::{init_cxt, num_app};
use crate::sync::UniProcSafeCell;
pub use context::TaskContext;
use lazy_static::*;
pub use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub struct TaskScheduler {
    num_app: usize,
    inner: UniProcSafeCell<TaskSchedulerInner>,
}

struct TaskSchedulerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
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
        panic!("Unreachable: switch task here");
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
            let mut inner = self.inner.borrow_mut();
            let current = inner.cur_task;
            inner.tasks[next].status = TaskStatus::Running;
            inner.cur_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].cxt as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].cxt as *const TaskContext;
            // before this, we should drop local variables that must be dropped manually
            debug!(
                "shift to {} at 0x{:x}",
                next,
                inner.tasks[next].cxt.sp() as usize
            );
            drop(inner);
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // go back to user mode
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
}

lazy_static! {
    pub static ref TASK_SCHED: TaskScheduler = {
        let mut tasks = [TaskControlBlock {
            cxt: TaskContext::from_zero(),
            status: task::TaskStatus::UnInit,
        }; MAX_APP_NUM];

        for (i, task) in tasks.iter_mut().enumerate() {
            task.cxt = TaskContext::with_restore(init_cxt(i));
            task.status = TaskStatus::Ready;
        }

        let inner = TaskSchedulerInner { tasks, cur_task: 0 };
        let num_app = num_app();

        TaskScheduler {
            num_app,
            inner: UniProcSafeCell::new(inner),
        }
    };
}

/// start running apps
pub fn start() {
    TASK_SCHED.start();
}

/// stop cur app (make it sleep and change cpu to other app)
pub fn cur_suspend() {
    TASK_SCHED.cur_suspend();
}

/// run next app
pub fn run_next() {
    TASK_SCHED.run_next();
}

/// cur app finished or cause error and be killed
pub fn cur_exit() {
    TASK_SCHED.cur_exit();
}
