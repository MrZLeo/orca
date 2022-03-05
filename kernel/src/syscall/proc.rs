use crate::task::{self, processor, scheduler, task::ProcessControlBlock};
use crate::timer::time_ms;

pub fn sys_exit(exit_code: i32) -> ! {
    warn!("Application exitd with code {}", exit_code);
    // TODO
    panic!("Unreachable: app exited");
}

pub fn sys_yield() -> isize {
    task::suspend_cur_and_run_next();
    0
}

pub fn sys_time() -> isize {
    time_ms() as isize
}

// TODO
pub fn sys_fork() -> isize {
    let cur_task = processor::cur_task().unwrap();
    let child = ProcessControlBlock::fork(&cur_task);
    let pid = child.pid;

    // set child return code = 0
    child.borrow_mut().trap_cxt().x[10] = 0;
    scheduler::add_task(child);

    pid.0 as isize
}

// TODO
pub fn sys_exec() -> isize {
    0
}
