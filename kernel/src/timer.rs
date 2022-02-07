use crate::{config::CLOCK_FREQ, sbi::set_timer};
use riscv::register;

const TICKS_PER_SEC: usize = 100;
const MSEC_PER_SEC: usize = 1000;
const MICRO_PER_SEC: usize = 1_000_000;

pub fn time() -> usize {
    register::time::read()
}

pub fn time_ms() -> usize {
    register::time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
}

pub fn time_us() -> usize {
    register::time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}

pub fn set_strigger() {
    set_timer(time() + CLOCK_FREQ / TICKS_PER_SEC)
}
