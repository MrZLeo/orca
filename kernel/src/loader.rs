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
