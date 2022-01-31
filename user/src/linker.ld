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

    . = ALIGN(4K);

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    .data : {
        *(.data .data.*)
        *(.sdata sdata.*)
    }

    . = ALIGN(4K);

    .bss : {
        sbss = .; /* start of bss */
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .; /* end of bss */


    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}
