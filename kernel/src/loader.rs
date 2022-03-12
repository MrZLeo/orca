use alloc::vec::Vec;

use crate::console::{println_with_color, YELLOW};

extern "C" {
    fn _num_app();
}

#[inline]
fn apps_ptr() -> *const usize {
    (_num_app as usize) as *const usize
}

#[inline]
pub fn num_app() -> usize {
    unsafe { apps_ptr().read_volatile() }
}

pub fn get_app_data(app_id: usize) -> &'static [u8] {
    let app_ptr = (_num_app as usize) as *const usize;
    let num_app = num_app();
    let app_start = unsafe { core::slice::from_raw_parts(app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}

lazy_static! {
    static ref APP_NAMES: Vec<&'static str> = {
        let num_app = num_app();
        extern "C" {
            fn _app_names();
        }
        let mut start = _app_names as usize as *const u8;
        let mut v = Vec::new();
        unsafe {
            for _ in 0..num_app {
                let mut end = start;
                while end.read_volatile() != b'\0' {
                    end = end.add(1);
                }
                let slice = core::slice::from_raw_parts(start, end as usize - start as usize);
                let str = core::str::from_utf8(slice).unwrap();
                v.push(str);
                start = end.add(1);
            }
        }
        v
    };
}

pub fn app_from_name(name: &str) -> Option<&'static [u8]> {
    debug!("stop point2");
    APP_NAMES
        .iter()
        .enumerate()
        .find(|&(_idx, n)| *n == name)
        .map(|(idx, _n)| get_app_data(idx))
}

pub fn list_app() {
    println!("---------- APPS ----------");
    APP_NAMES
        .iter()
        .for_each(|name| println_with_color(name, YELLOW));
    println!("---------- END  ----------");
}
