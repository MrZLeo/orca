// stack
pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
pub const APP_BASE_ADDR: usize = 0x1_0000;
pub const APP_SIZE_LIMIT: usize = 0x20000;

// qemu clock frequncy: 12.5MHz
pub const CLOCK_FREQ: usize = 12_500_000;

// page
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_OFFSET: usize = 12;

// memory
pub const MEMORY_END: usize = 0x8080_0000;
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

extern "C" {
    pub fn skernel();
    pub fn ekernel();
    pub fn stext();
    pub fn etext();
    pub fn srodata();
    pub fn erodata();
    pub fn sdata();
    pub fn edata();
    pub fn sbss_with_stack();
    pub fn sbss();
    pub fn ebss();
    pub fn strampoline();
}
