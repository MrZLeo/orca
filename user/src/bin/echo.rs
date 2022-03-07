#![no_std]
#![no_main]

use user_lib::readline;

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> i32 {
    let str = readline();
    println!("{}", str);
    0
}
