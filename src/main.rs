#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_item;
mod sbi;

use core::arch::global_asm;
use sbi::shutdown;

global_asm!(include_str!("entry.S"));

#[no_mangle]
pub fn __main() {
    clear_bss();
    sys_info();
    shutdown();
    // loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|x| unsafe { (x as *mut u8).write_volatile(0) });
}

fn sys_info() {
    println!(r"   ___   ____    ____     _    ");
    println!(r"  / _ \ |  _ \  / ___|   / \   ");
    println!(r" | | | || |_) || |      / _ \  ");
    println!(r" | |_| ||  _ < | |___  / ___ \ ");
    println!(r"  \___/ |_| \_\ \____|/_/   \_\");
    println!(r"                               ");
}

#[cfg(test)]
mod test {}
