use crate::fs::stdio;
use crate::fs::File;
use alloc::vec;
use core::cell::RefMut;

use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};

use crate::{
    config::TRAP_CONTEXT,
    mm::{
        address::{PhysPageNum, VirtAddr},
        memory_set::{MemorySet, KERNEL_SPACE},
    },
    sync::UniProcSafeCell,
    trap::{trap_handler, TrapContext},
};

use super::{
    kernel_stack::KernelStack,
    pid::{pid_alloc, Pid},
    TaskContext,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// Task that just created
    UnInit,
    /// Task is ready to run
    Ready,
    /// Task is runnning
    Running,
    /// Task that already completed or killed
    Exited,
    /// exited but not free memory yet
    Zombie,
}

pub struct ProcessControlBlock {
    pub pid: Pid,
    pub kernel_stack: KernelStack,
    pub inner: UniProcSafeCell<ProcessControlBlockInner>,
}

pub struct ProcessControlBlockInner {
    pub status: TaskStatus,
    pub cxt: TaskContext,
    pub memory_set: MemorySet,
    pub trap_cxt_ppn: PhysPageNum,
    pub base_size: usize,
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub children: Vec<Arc<ProcessControlBlock>>,
    pub fd_table: Vec<Option<Arc<dyn File + Send + Sync>>>,
    pub exit_code: i32,
}

impl ProcessControlBlockInner {
    pub fn trap_cxt(&self) -> &'static mut TrapContext {
        self.trap_cxt_ppn.as_mut()
    }

    pub fn user_token(&self) -> usize {
        self.memory_set.token()
    }

    pub fn status(&self) -> TaskStatus {
        self.status
    }

    pub fn is_zombie(&self) -> bool {
        self.status == TaskStatus::Zombie
    }

    pub fn alloc_fd(&mut self) -> usize {
        if let Some(fd) = (0..self.fd_table.len()).find(|fd| self.fd_table[*fd].is_none()) {
            fd
        } else {
            self.fd_table.push(None);
            self.fd_table.len() - 1
        }
    }
}

impl ProcessControlBlock {
    pub fn new(elf_data: &[u8]) -> Self {
        debug!("create PCB for initproc");
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);
        // green
        let trap_cxt_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let status = TaskStatus::Ready;
        let pid = pid_alloc();
        let kernel_stack = KernelStack::new(pid.0);
        let kernel_top = kernel_stack.top();

        let tcb = ProcessControlBlock {
            pid,
            kernel_stack,
            inner: UniProcSafeCell::new(ProcessControlBlockInner {
                status,
                cxt: TaskContext::with_trap_return(kernel_top),
                memory_set,
                trap_cxt_ppn,
                base_size: user_sp,
                parent: None,
                children: Vec::new(),
                exit_code: 0,
                fd_table: vec![
                    // 0 -> stdin
                    Some(Arc::new(stdio::Stdin)),
                    // 1 -> stdout
                    Some(Arc::new(stdio::Stdout)),
                    // 2 -> stderr
                    Some(Arc::new(stdio::Stdout)),
                ],
            }),
        };

        let trap_cxt = tcb.borrow_mut().trap_cxt();
        *trap_cxt = TrapContext::app_init_cxt(
            entry_point,
            user_sp,
            KERNEL_SPACE.borrow_mut().token(),
            kernel_top,
            trap_handler as usize,
        );

        tcb
    }

    pub fn borrow_mut(&self) -> RefMut<'_, ProcessControlBlockInner> {
        self.inner.borrow_mut()
    }

    pub fn getpid(&self) -> usize {
        self.pid.0
    }

    pub fn fork(parent: &Arc<ProcessControlBlock>) -> Arc<ProcessControlBlock> {
        let mut parent_inner = parent.borrow_mut();
        let memory_set = parent_inner.memory_set.clone();
        let trap_cxt_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();
        let pid = pid_alloc();
        let kernel_stack = KernelStack::new(pid.0);
        let kernel_stack_top = kernel_stack.top();

        // copy fd table
        let mut new_fd_table = Vec::new();
        for fd in parent_inner.fd_table.iter() {
            if let Some(file) = fd {
                new_fd_table.push(Some(file.clone()));
            } else {
                new_fd_table.push(None);
            }
        }

        let tcb = Arc::new(ProcessControlBlock {
            pid,
            kernel_stack,
            inner: UniProcSafeCell::new(ProcessControlBlockInner {
                status: TaskStatus::Ready,
                cxt: TaskContext::with_trap_return(kernel_stack_top),
                memory_set,
                trap_cxt_ppn,
                base_size: parent_inner.base_size,
                parent: Some(Arc::downgrade(parent)),
                children: Vec::new(),
                fd_table: new_fd_table,
                exit_code: 0,
            }),
        });

        // add children
        parent_inner.children.push(tcb.clone());

        // TODO: what this code about?
        let trap_cxt = tcb.borrow_mut().trap_cxt();
        trap_cxt.kernel_sp = kernel_stack_top;

        tcb
    }

    pub fn exec(&self, elf_data: &[u8]) {
        let (memory_set, sp, entry) = MemorySet::from_elf(elf_data);
        let trap_cxt_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap()
            .ppn();

        let mut inner = self.borrow_mut();
        inner.memory_set = memory_set;
        inner.trap_cxt_ppn = trap_cxt_ppn;

        let trap_cxt = inner.trap_cxt();
        *trap_cxt = TrapContext::app_init_cxt(
            entry,
            sp,
            KERNEL_SPACE.borrow_mut().token(),
            self.kernel_stack.top(),
            trap_handler as usize,
        )
    }
}
