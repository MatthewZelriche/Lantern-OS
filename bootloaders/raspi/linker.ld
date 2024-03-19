INCLUDE ../../../common.ld
ENTRY(kstart)

/*
   TODO: RESEARCH
   According to https://github.com/raspberrypi/firmware/issues/1193,
   naming the kernel kernel8.img will force arm_64bit = 1. And according to 
   https://forums.raspberrypi.com/viewtopic.php?t=328000#p1966797,
   if arm_64bit = 1, then the RPI firmware will look only at 0x80000 for the kernel entry point
   This contradicts other information online that suggests we have to use 0x8000 for the RPI3. 
   It's difficult to tell for sure who is right since I have no access to RPI3 hardware.
   qemu always seems to load the kernel to 0x80000, which may or may not reflect the real hardware.
   For now, we assume that both RPI3 and RPI4 will load at 0x80000 on real hardware
*/
__START_ADDR = 0x80000;

SECTIONS {
   . = __START_ADDR;
   __BOOTLOADER_START = .;
   .text : {
    *(.text.boot) /* This section is manually set in main.S to ensure entry point is at __START_ADDR */
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
   
   /* Put our stack on the next page after the kernel */
   . = ALIGN(__PG_SIZE);
   __BOOTLOADER_END = .;
   __STACK_END = .;
   . += __PG_SIZE * 2;
   __STACK_START = .;

   . = ALIGN(__PG_SIZE);
   __KERNEL_PHYS_START = .;
    .kernel : {
      *(.kernel)
      KEEP(*(.kernel))  /* To prevent the linker from stripping this unreferenced section out */
    }
   . = ALIGN(__PG_SIZE);
    __KERNEL_PHYS_END = .;
}