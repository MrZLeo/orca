OUTPUT_ARCH( "riscv" )
ENTRY( _start )
BASE_ADDRESS = 0x10000;

SECTIONS
{
    . = BASE_ADDRESS;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    /* .text   => R | X
     * .rodata => R
     * .data   => R | W
     * they have different permission, so they have to be aligned
     */
    . = ALIGN(4K);
    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4K);
    .data : {
        *(.data .data.*)
    }
    sbss = .;
    .bss : {
        *(.bss .bss.*)
    }
    ebss = .;
    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}
