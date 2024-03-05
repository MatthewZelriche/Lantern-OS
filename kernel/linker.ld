INCLUDE ../../../common.ld

ENTRY(kmain)

/* TODO: FOR TESTING ONLY: CHANGE WHEN WE ARE READY FOR HIGHER HALF */
__START_ADDR = 0x1488000;

SECTIONS {
   . = __START_ADDR;
   .text : {
    *(.text.boot) /* This section is manually set in main.rs to ensure entry point is at __START_ADDR */
    *(.text)
    *(.text.*)
   }

   /* Ensure other sections occur after .text so that the entry point remains at the start of the flat binary */
   .data : { *(.data); *(.data.*) }
   .rodata : { *(.rodata); *(.rodata.*) }

   /* Ensure BSS is at the very end, so we don't have to store a zeroed section of the kernel image. */
   . = ALIGN(8);
   __BSS_START = .;
   .bss : {
      *(.bss)
      *(.bss.*)
   }
   . = ALIGN(8);  /* Ensure total size ends up a multiple of 8 for our bss zeroing asm code */
   __BSS_END = .;
}