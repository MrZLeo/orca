use crate::mm::address::VirtAddr;
use crate::mm::frame_allocator::*;
use crate::mm::memory_set::KERNEL_SPACE;
use crate::test::{test_assert, test_assert_eq, test_fn};
use alloc::vec::Vec;

const MM_TEST_NUM: usize = 4;

fn heap_test() {
    use alloc::boxed::Box;
    use alloc::vec;

    let a = Box::new(5);
    test_assert_eq(*a, 5);
    let b = vec![1, 2, 3];
    test_assert_eq(b.len(), 3);
    test_assert_eq(b[0], 1);
}

fn heap_test2() {
    use alloc::boxed::Box;
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
}

// import position of differnet sections
use crate::config::edata;
use crate::config::erodata;
use crate::config::etext;
use crate::config::sdata;
use crate::config::srodata;
use crate::config::stext;

pub fn remap_test() {
    let mut kernel_space = KERNEL_SPACE.borrow_mut();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    test_assert(
        !kernel_space
            .page_table_mut()
            .translate(mid_text.floor())
            .unwrap()
            .writable(),
    );
    test_assert(
        !kernel_space
            .page_table_mut()
            .translate(mid_rodata.floor())
            .unwrap()
            .writable(),
    );
    test_assert(
        !kernel_space
            .page_table_mut()
            .translate(mid_data.floor())
            .unwrap()
            .executable(),
    );
    test!("remap test...");
}
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for _ in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for _ in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    test!("frame allocator test...");
}
pub fn mm_test() {
    test!("Memory Test Start: Running {} test\n", MM_TEST_NUM);
    test!("heap test1...");
    test_fn(heap_test);
    test!("heap test2...");
    test_fn(heap_test2);
    test_fn(frame_allocator_test);
    test_fn(remap_test);
}
