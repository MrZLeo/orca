#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{time, user_yield};

#[no_mangle]
fn main() -> i32 {
    let current_timer = time();
    let wait_for = current_timer + 3000;
    while time() < wait_for {
        user_yield();
    }
    println!("Test sleep OK!");
    0
}
