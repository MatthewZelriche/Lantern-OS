.section ".text.boot"
.globl kstart
kstart:
   # Set up a temporary stack for the primary core before we transition over to rust code
   ldr x1, =__TEMP_STACK_START
   mov sp, x1

   # Zero BSS, if such a section exists
   ldr x2, =__BSS_END
   ldr x3, =__BSS_START
   zero_bss:
      # Check if we zeroed out all the BSS data
      cmp x2, x3
      beq to_rust
      # Zero out 8 bytes of BSS and loop
      str xzr, [x3]
      add x3, x3, #8
      bl zero_bss

   to_rust:
      # For now, just loads a linker symbol into a register and loops
      # so we can view if the code executed successfully in QEMU
      ldr x1, =__RPI_VER
      loop:
      bl loop