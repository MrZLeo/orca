.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    csrrw sp, sscratch, sp
    addi sp, sp, -34*8
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

    mv a0, sp
    call trap_handler # trap_handler(ctx: &context)

__restore:
    mv sp, a0

    # reverse order: start from sscratch, sepc, sstatus
    ld t0, 33*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sscratch, t2
    csrw sepc, t1
    csrw sstatus, t0

    # restore greneral purpose registers except sp/tp
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr

    addi sp, sp, 34*8

    # change stack (sp -> user stack; sscratch -> kernel)
    csrrw sp, sscratch, sp
    sret
