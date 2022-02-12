/// stdin: 0
/// stdout: 1
/// stderr: 2
const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;
const FD_STDERR: usize = 2;

use crate::{mm::page_table::translated_byte_buffer, task::cur_user_token};

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(cur_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => panic!("Unknown fd: {}", fd),
    }
}
