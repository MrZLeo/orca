mod context;

use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::syscall::syscall;
use crate::task::processor::cur_trap_cxt;
use crate::task::suspend_cur_and_run_next;
use crate::timer::set_strigger;
pub use context::TrapContext;
use core::arch::{asm, global_asm};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    set_kernel_trap_entry();
}

#[no_mangle]
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let cxt = cur_trap_cxt();
    let scause = scause::read();
    let stval = stval::read();

    // handler exception by scause
    match scause.cause() {
        // syscall interface
        Trap::Exception(Exception::UserEnvCall) => {
            cxt.sepc += 4;
            let res = syscall(cxt.x[17], [cxt.x[10], cxt.x[11], cxt.x[12]]) as usize;
            // current context may be change by `exec`, so we have to get context again
            cxt = cur_trap_cxt();
            cxt.x[10] = res as usize;
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
            suspend_cur_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}",
                scause.cause(),
                stval
            )
        }
    }

    trap_return();
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

pub fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel")
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cxt_ptr = TRAP_CONTEXT;
    let user_satp = cur_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!("fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cxt_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}
