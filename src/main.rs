#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_item;
mod orca_logo;
mod sbi;

use core::arch::global_asm;
use lang_item::panic;
use sbi::shutdown;

global_asm!(include_str!("entry.S"));

#[no_mangle]
pub fn __main() {
    clear_bss();
    sys_info();
    panic!("test panic!");
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
    print!("{}", orca_logo::ORCA_LOGO);
}

#[cfg(test)]
mod test {}
