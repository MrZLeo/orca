#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(unused)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod config;
mod drivers;
mod fs;
mod lang_item;
mod mm;
mod orca_logo;
mod sbi;
mod sync;
mod syscall;
mod task;
mod test;
mod timer;
mod trap;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;

// import position of differnet sections
use crate::config::ebss;
use crate::config::edata;
use crate::config::ekernel;
use crate::config::erodata;
use crate::config::etext;
use crate::config::sbss;
use crate::config::sbss_with_stack;
use crate::config::sdata;
use crate::config::skernel;
use crate::config::srodata;
use crate::config::stext;
use crate::fs::inode::list_apps;
use crate::fs::inode::ROOT_INODE;

use core::arch::global_asm;

extern crate alloc;

// entry of orca OS
// bootloader will shift $pc to entry.S
// and entry.S will call __main here
global_asm!(include_str!("entry.S"));

#[no_mangle]
pub fn __main() {
    clear_bss();

    /* init */
    mm::init();

    debug!("create initproc");
    task::add_initproc();

    debug!("trap init");
    trap::init();

    debug!("start timer");
    trap::enable_timer_interrupt();
    timer::set_strigger();

    /* show system info */
    sys_info();
    kernel!("Hello World!");

    /* enter kernel test module
     * ONLY enter when testing kernel */
    #[cfg(feature = "kernel_test")]
    {
        test::main();
    }

    /* start schedule process */
    list_apps();
    task::run();

    panic!("unreachable: __main ended");
}

fn clear_bss() {
    (sbss as usize..ebss as usize).for_each(|x| unsafe { (x as *mut u8).write_volatile(0) });
}

fn sys_info() {
    print!("\x1b[1m{}\x1b[0m", orca_logo::ORCA_LOGO);
    info!(
        "kernel range [{:#x}, {:#x}]",
        skernel as usize, ekernel as usize
    );
    info!(".text [{:#x}, {:#x}]", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x}]", sdata as usize, edata as usize);
    info!(
        ".bss [{:#x}, {:#x}]",
        sbss_with_stack as usize, ebss as usize
    );
}
