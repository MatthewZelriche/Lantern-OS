INCLUDE ../../../common.ld

SECTIONS {
   /* 
      RPI expects boot code at a specific address. 
      This address is set by build.rs for the raspi crate
    */
   . = __START_ADDR;
   .text : {
    *(.text.boot) /* This section is manually set in main.S to ensure entry point is at __START_ADDR */
    *(.text)
    *(.text.*)
   }
}