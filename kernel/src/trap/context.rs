use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct Context {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl Context {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_cxt(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut ctx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        ctx.set_sp(sp);
        ctx
    }
}
