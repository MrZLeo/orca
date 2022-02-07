pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const MAX_APP_NUM: usize = 4;
pub const APP_BASE_ADDR: usize = 0x8040_0000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

// qemu clock frequncy: 12.5MHz
pub const CLOCK_FREQ: usize = 12_500_000;
