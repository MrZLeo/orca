#![allow(clippy::print_with_newline)]
extern crate alloc;
use crate::{read, write};
use alloc::string::String;
use core::fmt::{self, Write};

/// std input & output & error
const STDIN: usize = 0;
const STDOUT: usize = 1;
const STDERR: usize = 2;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

pub struct Color(usize);

// hex presentation of color
pub const BLACK: Color = Color(30);
pub const RED: Color = Color(31);
pub const GREEN: Color = Color(32);
pub const YELLOW: Color = Color(33);
pub const BLUE: Color = Color(34);
pub const PURPLE: Color = Color(35);
pub const DEEP_GREEN: Color = Color(36);
pub const WHITE: Color = Color(37);

pub fn print_with_color(s: &str, color: Color) {
    print!("\x1b[{}m{}\x1b[0m", color.0, s);
}

pub fn println_with_color(s: &str, color: Color) {
    println!("\x1b[{}m{}\x1b[0m", color.0, s);
}

pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    read(STDIN, &mut c);
    c[0]
}

pub const LF: u8 = 0x0a;
pub const CR: u8 = 0x0d;
pub const DL: u8 = 0x7f;
pub const BS: u8 = 0x08;

fn getline() -> String {
    let mut line = String::new();
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                // get a '\n'
                print!("\n");
                if !line.is_empty() {
                    line.push('\0');
                    return line;
                }
            }
            BS | DL => {
                if !line.is_empty() {
                    // print a ' ' to cover deleted char
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}
