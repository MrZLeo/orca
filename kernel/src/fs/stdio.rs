use crate::{sbi::consolo_getchar, task::suspend_cur_and_run_next};

use super::File;

/// standard input
pub struct Stdin;

/// standard output
pub struct Stdout;

impl File for Stdin {
    fn read(&self, mut buf: crate::mm::page_table::UserBuf) -> usize {
        // read a char once
        assert_eq!(buf.len(), 1);

        // busy quiering
        let mut c;

        c = consolo_getchar();
        while c == 0 {
            // don't get a char, switch task and try latter
            suspend_cur_and_run_next();
            c = consolo_getchar();
        }

        let ch = c as u8;
        unsafe {
            buf.buffers[0].as_mut_ptr().write_volatile(ch);
        }
        1
    }

    fn write(&self, buf: crate::mm::page_table::UserBuf) -> usize {
        panic!("can't write to stdin");
    }

    fn readable(&self) -> bool {
        true
    }

    fn writeable(&self) -> bool {
        false
    }
}

impl File for Stdout {
    fn read(&self, buf: crate::mm::page_table::UserBuf) -> usize {
        panic!("can't read from stdout");
    }

    fn write(&self, buf: crate::mm::page_table::UserBuf) -> usize {
        for buffer in buf.buffers.iter() {
            print!("{}", core::str::from_utf8(buffer).unwrap());
        }
        buf.len()
    }

    fn readable(&self) -> bool {
        false
    }

    fn writeable(&self) -> bool {
        true
    }
}
