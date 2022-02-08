#![allow(unused)]
use crate::sbi::console_putchar;
use core::fmt::{self, Write};

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
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
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(concat!($fmt, "\n") $(, $($arg)+)?)
        );
    };
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

pub fn test_ok() {
    println_with_color("Ok", GREEN);
}

pub fn test_err() {
    println_with_color("Err", RED);
}

/// color for logo
/// - error: red
/// - info: purple
/// - warn: yellow
/// - debug: green
/// - trace: white
/// - kernel: deep_green
/// - test: blue
#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(
                concat!(
                    "\x1b[31;1m",
                    "[ERROR]\x1b[0m",
                    "\x1b[31m ",
                    $fmt,
                    "\x1b[0m\n")
                    $(, $($arg)+)?
            )
        );
    };
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(
                concat!(
                    "\x1b[35;1m",
                    "[INFO]\x1b[0m",
                    "\x1b[35m ",
                    $fmt,
                    "\x1b[0m\n")
                    $(, $($arg)+)?
            )
        );
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(
                concat!(
                    "\x1b[33;1m",
                    "[WARN]\x1b[0m",
                    "\x1b[33m ",
                    $fmt,
                    "\x1b[0m\n")
                    $(, $($arg)+)?
            )
        );
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(
                concat!(
                    "\x1b[32;1m",
                    "[DEBUG]\x1b[0m",
                    "\x1b[32m ",
                    $fmt,
                    "\x1b[0m\n")
                    $(, $($arg)+)?
            )
        );
    };
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(
                concat!(
                    "\x1b[1m",
                    "[TRACE]\x1b[0m",
                    "\x1b[2m ",
                    $fmt,
                    "\x1b[0m\n")
                    $(, $($arg)+)?
            )
        );
    };
}

#[macro_export]
macro_rules! kernel {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(
                concat!(
                    "\x1b[36;1m",
                    "[KERNEL]\x1b[0m",
                    "\x1b[36m ",
                    $fmt,
                    "\x1b[0m\n")
                    $(, $($arg)+)?
            )
        );
    };
}

#[macro_export]
macro_rules! test {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(
            format_args!(
                concat!(
                    "\x1b[34;1m",
                    "[TEST]\x1b[0m",
                    "\x1b[34m ",
                    $fmt,
                    "\x1b[0m")
                    $(, $($arg)+)?
            )
        );
    };
}
