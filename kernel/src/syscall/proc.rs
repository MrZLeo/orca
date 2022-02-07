use crate::task::{cur_exit, cur_suspend, run_next};
use crate::timer::time_ms;

pub fn sys_exit(exit_code: i32) -> ! {
    warn!("[kernel] Application exitd with code {}", exit_code);
    cur_exit();
    run_next();
    panic!("Unreachable: app exited");
}

pub fn sys_yield() -> isize {
    cur_suspend();
    run_next();
    0
}

pub fn sys_time() -> isize {
    time_ms() as isize
}
