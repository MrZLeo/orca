use crate::console::test_ok;

fn heap_test() {
    use alloc::boxed::Box;
    extern "C" {
        fn sbss();
        fn ebss();
    }

    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);
}

pub fn mm_test() {
    test!("Memory Test Start\n");
    test!("heap test...");
    heap_test();
    test_ok();
}
