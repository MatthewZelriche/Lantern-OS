.section ".text.boot"
.globl kstart
kstart:
   # For now, just loads a linker symbol into a register and loops
   # so we can view if the code executed successfully in QEMU
   ldr x1, =__RPI_VER
   loop:
   bl loop