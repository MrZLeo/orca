#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{getpid, user_yield};

#[no_mangle]
pub fn main() -> i32 {
    println!("Hello, I am process {}.", getpid());
    for i in 0..5 {
        user_yield();
        println!("Back in process {}, iteration {}.", getpid(), i);
    }
    println!("yield pass.");
    0
}
