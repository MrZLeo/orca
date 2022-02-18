//! # Osh implementation
//!
//! osh is a general command shell for orca, it will follow architecture:
//! `OshLexer` --> `OshParser` --> `shell.rs`(binary program in `bin/`)

#![macro_use]
extern crate alloc;
use alloc::string::String;

const LF: u8 = 0x0a;
const CR: u8 = 0x0d;
const DL: u8 = 0x7f;
const BS: u8 = 0x08;

// TODO
pub enum Command {
    Bin(String),
}
pub fn get_command() -> Option<Command> {
    None
}
