use crate::batch::batch_schedule;

pub fn sys_exit(exit_code: i32) -> ! {
    warn!("[kernel] Application exitd with code {}", exit_code);
    batch_schedule();
}
