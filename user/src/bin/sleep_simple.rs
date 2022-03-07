#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{sleep, time};

#[no_mangle]
pub fn main() -> i32 {
    println!("into sleep test!");
    let start = time();
    println!("current time_msec = {}", start);
    sleep(100);
    let end = time();
    println!(
        "time_msec = {} after sleeping 100 ticks, delta = {}ms!",
        end,
        end - start
    );
    println!("r_sleep passed!");
    0
}
