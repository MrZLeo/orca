/// stdin: 0
/// stdout: 1
/// stderr: 2
const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;
const FD_STDERR: usize = 2;

use crate::mm::page_table::translated_byte_buffer;
use crate::sbi::consolo_getchar;
use crate::task::processor::{self, cur_user_token};
use crate::task::suspend_cur_and_run_next;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(processor::cur_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => panic!("Unknown fd: {}", fd),
    }
}

/// @return the len that read from `fd`
pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDIN => {
            let mut c;
            loop {
                c = consolo_getchar();
                if c == 0 {
                    suspend_cur_and_run_next();
                    continue;
                } else {
                    break;
                }
            }
            let ch = c as u8;
            let mut buffer = translated_byte_buffer(cur_user_token(), buf, len);
            unsafe {
                buffer[0].as_mut_ptr().write_volatile(ch);
            }
            1
        }
        _ => 0,
    }
}
