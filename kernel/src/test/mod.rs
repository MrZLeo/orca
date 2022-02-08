//! # Orca Test Architecture
//! This is a Test architecture for orca, which is simple but good enough to support
//! orca kernel test.
//!
//! ## Usage:
//! In test directory, you can design your own test module and bind it to `mod.rs`
//! It is better to name your module like `xx_test.rs`
//!
//! In your module, you should desigin a interface like `xx_test`, which contains your whole test procedure.
//! Pay attention that your test function must be use by `test_fn`, which can help us records test result.
//! After that, call it in `main` of `mod.rs`
//!
//! ## Test Functions:
//! Test functions is used like `assert!()` macro, but `assert` macro will panic if test don't
//! pass, which can't be use to records test result.
//!
//! ## Example:
//! 1. Define test module `mm_test.rs` in directory `test`
//! 2. Define test interface `mm_test`
//! 3. Define variable `MM_TEST_NUM` to store the number of test in this module
//! 4. Define two test function: `heap_test` and `heap_test2`
//! 5. In `mm_test()`, call test function:
//! ```rust
//! pub fn mm_test() {
//!     test!("Memory Test Start: Running {} test\n", MM_TEST_NUM);
//!     test!("heap test1...");
//!     test_fn(heap_test);
//!     test!("heap test2...");
//!     test_fn(heap_test2);
//! }
//! ```
//! 6. import my module in `mod.rs`, call the interface `mm_test`
//!

use core::borrow::BorrowMut;

use crate::{
    console::{print_with_color, println_with_color, test_err, test_ok, GREEN, RED},
    sbi::shutdown,
    sync::UniProcSafeCell,
};

use alloc::format;
use lazy_static::lazy_static;

mod mm_test;

pub fn main() -> ! {
    mm_test::mm_test();
    test!("--- TEST END ---\n");
    test!("Test Result: ");
    print_with_color(
        format!("success: {}, ", TEST_MANAGER.get_success()).as_str(),
        GREEN,
    );
    println_with_color(format!("fail: {}", TEST_MANAGER.get_fail()).as_str(), RED);
    shutdown();
}

struct TestManager {
    success: UniProcSafeCell<usize>,
    fail: UniProcSafeCell<usize>,
    stage: UniProcSafeCell<bool>,
}

impl TestManager {
    pub fn get_success(&self) -> usize {
        let s = self.success.borrow_mut();
        *s
    }

    pub fn get_fail(&self) -> usize {
        let f = self.fail.borrow_mut();
        *f
    }

    pub fn success(&self) {
        let mut s = self.success.borrow_mut();
        *s += 1;
        test_ok();
    }

    pub fn fail(&self) {
        let mut f = self.fail.borrow_mut();
        *f += 1;
        test_err();
    }

    pub fn test_fail(&self) {
        let mut s = self.stage.borrow_mut();
        *s = false;
    }
}

lazy_static! {
    static ref TEST_MANAGER: TestManager = TestManager {
        success: UniProcSafeCell::new(0),
        fail: UniProcSafeCell::new(0),
        stage: UniProcSafeCell::new(true)
    };
}

fn success() {
    TEST_MANAGER.success();
}

fn fail() {
    TEST_MANAGER.fail();
}

fn test_fail() {
    TEST_MANAGER.test_fail();
}

/// general purpose test assert functions
/// - test_assert_eq() -> assert_eq!()
/// - test_assert() -> assert!()
/// - test_assert_ne() -> assert_ne!()
/// This three functions can make sure that when assert fail, os won't panic immediately
/// Program will record fail and show to user when test ended.
pub fn test_assert_eq<T: PartialEq>(a: T, b: T) {
    if a != b {
        test_fail();
    }
}

pub fn test_assert(a: bool) {
    test_assert_eq(&a, &true);
}

pub fn test_assert_ne<T: PartialEq>(a: T, b: T) {
    test_assert(a != b);
}

pub fn test_fn<F: FnOnce()>(f: F) {
    let mut stage = TEST_MANAGER.stage.borrow_mut();
    *stage = true;
    drop(stage);
    f();
    let mut stage = TEST_MANAGER.stage.borrow_mut();
    if *stage {
        success();
    } else {
        fail();
    }
}
