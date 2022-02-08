use crate::sbi::shutdown;

mod mm_test;

pub fn main() -> ! {
    mm_test::mm_test();
    assert_eq!(1, 2);
    test!("--- TEST END ---\n");
    shutdown();
}
