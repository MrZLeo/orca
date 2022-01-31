#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(unused)]

#[macro_use]
mod console;
mod lang_item;
mod orca_logo;
mod sbi;

use console::{println_with_color, GREEN};
use core::arch::global_asm;
use lang_item::panic;
use sbi::shutdown;

global_asm!(include_str!("entry.S"));

#[no_mangle]
pub fn __main() {
    clear_bss();
    sys_info();
    shutdown();
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
    print!("{}", orca_logo::ORCA_LOGO);
    info!(
        "kernel range [{:#x}, {:#x}]",
        skernel as usize, ekernel as usize
    );
    info!(".text [{:#x}, {:#x}]", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x}]", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x}]", sbss as usize, ebss as usize);
}

#[cfg(test)]
mod test {}