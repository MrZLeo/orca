.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw sp, sscratch, sp
    sd x1, 1*8(sp)
    # skip x2(sp), save latter
    sd x3, 3*8(sp)
    # skip x4(tp), don't use

    # save x5-x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr

    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)

    csrr t2, sscratch
    sd t2, 2*8(sp)

    # load kernel_satp
    ld t0, 34*8(sp)
    # load trap_handler
    ld t1, 36*8(sp)
    # move to kernel_sp
    ld sp, 35*8(sp)


    # switch to kernel space
    csrw satp, t0
    # clear cache
    sfence.vma

    # jump to trap_handler
    jr t1

__restore:
    # a0: *TrapContext in user space(Constant)
    # a1: user space token
    csrw satp, a1
    sfence.vma
    csrw sscratch, a0

    mv sp, a0

    # sp -> *TrapContext | user space
    # reverse order: start from sscratch, sepc, sstatus
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)

    csrw sstatus, t0
    csrw sepc, t1

    # restore greneral purpose registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr


    # back to user stack
    ld sp, 2*8(sp)
    sret
