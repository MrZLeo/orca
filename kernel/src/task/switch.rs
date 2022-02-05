use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

use crate::task::TaskContext;

extern "C" {
    fn __switch(old_cxt: *mut TaskContext, new_cxt: *const TaskContext);
}
