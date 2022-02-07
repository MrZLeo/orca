OUTPUT_ARCH( "riscv" )
ENTRY( _entry )
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .; /* start of kernel */

    stext = .; /* start of text */
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .; /* end of text */

    srodata = .; /* start of read-only-data */
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .; /* end of read-only-data */

    sdata = .; /* start of data */
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .; /* end of data */

    .bss : {
        *(.bss.stack)
        sbss = .; /* start of bss */
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .; /* end of bss */

    ekernel = .; /* end of kernel */

    /DISCARD/ : {
        *(.eh_frame)
    }
}
