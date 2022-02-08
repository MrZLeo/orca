use crate::console::test_ok;
use crate::test::{test_assert, test_assert_eq, test_fn};

const MM_TEST_NUM: usize = 2;

fn heap_test() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }

    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    test_assert_eq(*a, 5);
    let mut b = Vec::new();
    b.push(1);
    b.push(2);
    b.push(3);
    test_assert_eq(b.len(), 3);
    test_assert_eq(b[0], 1);
}

fn heap_test2() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    test_assert_eq(*a, 5);
    test_assert(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);
    let mut v: Vec<usize> = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    for (i, val) in v.iter().take(500).enumerate() {
        test_assert_eq(*val, i);
    }
    test_assert(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    test_assert_eq(1, 2);
}

pub fn mm_test() {
    test!("Memory Test Start: Running {} test\n", MM_TEST_NUM);
    test!("heap test1...");
    test_fn(heap_test);
    test!("heap test2...");
    test_fn(heap_test2);
}
