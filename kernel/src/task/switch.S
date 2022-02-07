.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm

    .section .text
    .globl __switch
# __switch(old_cxt: &mut TaskContext, new_cxt: &TaskContext)
__switch:

    # store gengeral-prupose registers of old context
    # we just need to consider s0-s11, ra, sp, other registers will be store
    # when `fn __switch` be called (code generate by compiler)
    sd sp, 8(a0)
    sd ra, 0(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr

    # load general-purpose registers of new context
    ld ra, 0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr
    ld sp, 8(a1)
    ret
