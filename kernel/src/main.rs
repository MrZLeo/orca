#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(unused)]

#[macro_use]
mod console;
mod batch;
mod config;
mod lang_item;
mod load;
mod orca_logo;
mod sbi;
mod sync;
mod syscall;
mod trap;

use console::{println_with_color, GREEN};
use core::arch::global_asm;
use lang_item::panic;
use sbi::shutdown;

global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn __main() {
    clear_bss();
    sys_info();
    kernel!("Hello World!");
    trap::trap_init();
    batch::batch_init();
    batch::batch_schedule();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|x| unsafe { (x as *mut u8).write_volatile(0) });
}

fn sys_info() {
    extern "C" {
        fn skernel();
        fn ekernel();
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
    }
    print!("\x1b[1m{}\x1b[0m", orca_logo::ORCA_LOGO);
    info!(
        "kernel range [{:#x}, {:#x}]",
        skernel as usize, ekernel as usize
    );
    info!(".text [{:#x}, {:#x}]", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x}]", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x}]", sbss as usize, ebss as usize);
}
