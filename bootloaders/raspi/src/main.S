.section ".text.boot"
.globl kstart
kstart:
   # Set up a temporary stack for the primary core before we transition over to rust code
   ldr x1, =__STACK_START
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
      # Prep the switch from EL2 -> EL1
      # We need to set up some register state to configure how our eret will function
      # First we fake a program status where we are telling the cpu to "return" to EL1, using SP_EL1
      # We also inform the cpu that all interrupts in EL1 should be masked (we are not yet set up for them)
      ldr x9, =453
      msr spsr_el2, x9
      # We must also disable trap interrupts for vectorized and floating point instructions in EL1, otherwise
      # we will trigger an exception return back to EL2 and crash the CPU (because EL2 will never have an 
      # exception handler installed)
      ldr x9, =3145728
      msr cpacr_el1, x9	
      # In addition, we have to make sure the EL1 stack pointer is valid
      mov x9, xzr
      mov x9, sp
      msr sp_el1, x9
      # Next inform the CPU that we want EL1 to be running in 64 bit mode
      ldr x9, =2147483648 
      msr hcr_el2, x9
      # TODO: When we enable the MMU, there will be some additional registers we have to set here!
      # TODO
      # Finally, before we "return", we need to specify the address we are returning to
      adr x9, bootloader_main
      msr elr_el2, x9
      # Off we go!      
      # When jumping to rust entry point function: 
      # x0 contains the address of the start of the DTB
      eret