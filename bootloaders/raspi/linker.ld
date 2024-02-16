INCLUDE ../../../common.ld

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
   /* 
      RPI expects boot code at a specific address. 
      This address is set by build.rs for the raspi crate
    */
   . = __START_ADDR;
   __TEMP_STACK_START = .; /* Specify a temporary stack region before the entry point */
   .text : {
    *(.text.boot) /* This section is manually set in main.S to ensure entry point is at __START_ADDR */
    *(.text)
    *(.text.*)
   }

   /* Ensure other sections occur after .text so that the entry point remains at the start of the flat binary */
   .data : { *(.data); *(.data.*) }
   .rodata : { *(.rodata); *(.rodata.*) }

   /* Ensure BSS is at the very end, so we don't have to store a zeroed section of the kernel image. */
   __BSS_START = .;
   .bss : {
      *(.bss)
      *(.bss.*)
   }
   . = ALIGN(8); /* Align to an 8 byte boundary because we use str to zero 8 bytes at a time in asm */
   __BSS_END = .;
}