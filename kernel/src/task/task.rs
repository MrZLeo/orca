use crate::{
    config::{kernel_stack_position, TRAP_CONTEXT},
    mm::{
        address::{PhysPageNum, VirtAddr},
        memory_set::{MapPermission, MemorySet, KERNEL_SPACE},
    },
    trap::{trap_handler, TrapContext},
};

use super::TaskContext;

#[derive(Clone, Copy, PartialEq)]
pub enum TaskStatus {
    /// Task that just created
    UnInit,
    /// Task is ready to run
    Ready,
    /// Task is runnning
    Running,
    /// Task that already completed or killed
    Exited,
}

pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub cxt: TaskContext,
    pub memory_set: MemorySet,
    pub trap_cxt_ppn: PhysPageNum,
    pub base_size: usize,
}

impl TaskControlBlock {
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        let trap_cxt_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let status = TaskStatus::Ready;
        let (kernel_bottom, kernel_top) = kernel_stack_position(app_id);
        KERNEL_SPACE.borrow_mut().insert_framed_area(
            kernel_bottom.into(),
            kernel_top.into(),
            MapPermission::R | MapPermission::W,
        );
        let mut tcb = Self {
            status,
            cxt: TaskContext::with_trap_return(kernel_top),
            memory_set,
            trap_cxt_ppn,
            base_size: user_sp,
        };

        let trap_cxt = tcb.trap_cxt();
        *trap_cxt = TrapContext::app_init_cxt(
            entry_point,
            user_sp,
            KERNEL_SPACE.borrow_mut().token(),
            kernel_top,
            trap_handler as usize,
        );

        tcb
    }

    pub fn trap_cxt(&mut self) -> &'static mut TrapContext {
        self.trap_cxt_ppn.as_mut()
    }

    pub fn user_token(&self) -> usize {
        self.memory_set.token()
    }
}
