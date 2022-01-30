    .section .text.entry
    .globl _entry
_entry:
    la sp, stack0_top
    call __main

# spin:
#     j spin


    .section .bss.stack
    .globl stack0
stack0:
    .space 4096 * 16
    .globl stack0_top
stack0_top:
