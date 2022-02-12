mod fs;
mod proc;

/// syscall number
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_TIME: usize = 169;

use fs::*;
pub use proc::*;

/// general syscall implementation
pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_TIME => sys_time(),
        _ => panic!("Unsupported syscall id:{}", id),
    }
}
