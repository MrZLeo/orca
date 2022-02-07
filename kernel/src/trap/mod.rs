mod context;

use crate::syscall::syscall;
use crate::task::{cur_exit, cur_suspend, run_next};
use crate::timer::set_strigger;
pub use context::TrapContext;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();

    // handler exception by scause
    match scause.cause() {
        // syscall interface
        Trap::Exception(Exception::UserEnvCall) => {
            ctx.sepc += 4;
            ctx.x[10] = syscall(ctx.x[17], [ctx.x[10], ctx.x[11], ctx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("[kernel] Fage fault in application, kernel will kill it");
            cur_exit();
            run_next();
        }
        Trap::Exception(Exception::IllegalInstruction)
        | Trap::Exception(Exception::InstructionFault) => {
            error!("[kernel] Illegal Instruction in application, kernel will kill it");
            cur_exit();
            run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_strigger();
            cur_suspend();
            run_next();
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

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}
