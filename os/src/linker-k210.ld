OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80020000;

SECTIONS
{
    . = BASE_ADDRESS;
    start_kernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        . = ALIGN(16K);
        strampoline = .;
        *(.text.trampoline);
        . = ALIGN(16K);
        etrampoline = .;
        *(.text .text.*)
    }

    . = ALIGN(16K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(16K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(16K);
    edata = .;
    sbss_with_stack = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(16K);
    ebss = .;
    end_kernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}