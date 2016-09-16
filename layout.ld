_data_load = LOADADDR(.data);

ENTRY(main)

MEMORY
{
    rom(RX)     : ORIGIN = 0x08000000, LENGTH = 1024K
    ram_c(WAIL) : ORIGIN = 0x10000000, LENGTH = 64K
    ram(WAIL)   : ORIGIN = 0x20000000, LENGTH = 112K + 16K
    /* ram(WAIL) : ORIGIN = 0x2001C000, LENGTH = 16K */
}

__STACK_BASE = ORIGIN(ram_c) + LENGTH(ram_c);

__aeabi_unwind_cpp_pr0 = abort;
__aeabi_unwind_cpp_pr1 = abort;
__aeabi_unwind_cpp_pr2 = abort;
__exidx_start          = abort;
__exidx_end            = abort;

/*__aeabi_memclr4 = __aeabi_memclr;*/

SECTIONS
{
    .vector : ALIGN(4)
    {
        FILL(0xff)

        KEEP(*(.isr_vector))
        KEEP(*(.isr_vector_nvic))
    } > rom

    .text : ALIGN(4)
    {
        FILL(0xff)
        *(.text*)
        *(.rodata .rodata.*)
    } > rom

    .data : ALIGN(4)
    {
        _data = .;

        *(SORT_BY_ALIGNMENT(.data*))
        . = ALIGN(4);

        _edata = .;
    } > ram AT>rom = 0xff

    .bss : ALIGN(4)
    {
        _bss = .;

        *(.bss*)
        *(COMMON)
        . = ALIGN(4);

        _ebss = .;
    } > ram

    /DISCARD/ :
    {
        *(.glue_7*)  /* arm-thumb interworking */
        *(.v4_bx)  /* ARMv4 interworking fixup for missing BX */
        *(.vfp11_veneer)  /* VFP11 bugfixes s.a. http://sourceware.org/ml/binutils/2006-12/msg00196.html */
        *(.iplt .igot.plt)  /* STT_GNU_IFUNC symbols */
        *(.rel.*)  /* dynamic relocations */
        *(.ARM.exidx*) /* index entries for section unwinding */
        *(.ARM.extab*) /* exception unwinding information */
/*        *(.debug_gdb_scripts)*/
    }
}
