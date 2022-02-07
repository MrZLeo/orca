use crate::task::TaskContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

extern "C" {
    pub fn __switch(old_cxt: *mut TaskContext, new_cxt: *const TaskContext);
}
