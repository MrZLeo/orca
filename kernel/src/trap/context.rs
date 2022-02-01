use riscv::register::sstatus::Sstatus;

#[repr(C)]
pub struct Context {
    pub x: [usize; 32],
    pub xstatus: Sstatus,
    pub sepc: usize,
}

impl Context {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
}
