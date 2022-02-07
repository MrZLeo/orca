OUTPUT_ARCH( "riscv" )
ENTRY( _start )
BASE_ADDRESS = 0x80400000;

SECTIONS
{
    . = BASE_ADDRESS;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    .bss : {
        sbss = .; /* start of bss */
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .; /* end of bss */
    }
    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}
