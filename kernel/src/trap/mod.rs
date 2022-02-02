mod context;

use crate::batch::batch_schedule;
use crate::syscall::syscall;
pub use context::Context;
use core::arch::global_asm;
use riscv::register::{
    scause::{self, Exception, Trap},
    stval, stvec,
    utvec::TrapMode,
};

global_asm!(include_str!("trap.S"));

pub fn trap_init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(ctx: &mut Context) -> &mut Context {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            ctx.sepc += 4;
            ctx.x[10] = syscall(ctx.x[17], [ctx.x[10], ctx.x[11], ctx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("[kernel] Fage fault in application, kernel will kill it");
            batch_schedule();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] Illegal Instruction in application, kernel will kill it");
            batch_schedule();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}",
                scause.cause(),
                stval
            )
        }
    }
    ctx
}
