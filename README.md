# LanternOS

LanternOS is a hobbyist OS for the Raspberry Pi 3 and 4 SBCs.

## Building

First you need to install a few build dependencies:

```
rustup component add llvm-tools-preview
cargo install --force cargo-make
```

When building for raspberry pi 3, simply perform the following make command: 

```
cargo make raspi3-qemu
```

If you would like to build and run for RPI4 on Qemu, you will need to compile qemu from source with experimental 
patch support for the model 4, as it is not yet merged into the stable branch. Doing so is beyond the scope 
of these build instructions. Once you have a version of qemu with 
raspi4 support, execute the following:

```
export QEMU_PATH=/my/qemu/path/
cargo make raspi4-qemu
```

When switching between building for raspi3 and raspi4, it's strongly advised to perform a clean first:

```
cargo make clean
```

## Roadmap
- [X] Print Hello World with UART
- [X] Implement physical page frame allocator for bootloader
- [X] Enable the MMU to an identity mapping
- [X] Adjust virtual memory to load kernel in the higher half
- [X] Parse dtb and construct a memory map
- [ ] Implement synchronization primitives with MMU support (mutex, barrier)
- [ ] Properly transfer control from bootloader to kernel entry point on main core
- [ ] Create device driver for hardware timer
- [ ] Initialize secondary cores
- [ ] Implement kernel heap and enable alloc crate for kernel
- [ ] Framebuffer driver for printing to the screen 

## Sample Output
```
PL011 UART0 Device Driver initialized
Reserved range AB000 - AAB000 for bootloader frame allocation
Temporarily identity mapping first 4 GiB of address space
Mapped kernel to range 0xFFFF000000000000 - 0xFFFF000000001000
Mapped stack to range 0xFFFF000000001000 - 0xFFFF000000003000
Linearly mapped physical memory to range 0xFFFF000040000000 - 0xFFFF000240000000
Printing physical memory map:

Page size:      4.000 KiB
Free Memory:    1.937 GiB
Free Pages:     507895
Reserved Pages: 9
Type: Reserved   | 0x0000000000000000 - 0x0000000000001000 | 4.000 KiB
Type: Free       | 0x0000000000001000 - 0x00000000000a8000 | 668.000 KiB
Type: Stack      | 0x00000000000a8000 - 0x00000000000aa000 | 8.000 KiB
Type: Kernel     | 0x00000000000aa000 - 0x00000000000ab000 | 4.000 KiB
Type: Reserved   | 0x00000000000ab000 - 0x00000000000b0000 | 20.000 KiB
Type: Free       | 0x00000000000b0000 - 0x000000003c000000 | 959.312 MiB
Type: Free       | 0x0000000040000000 - 0x0000000080000000 | 1.000 GiB

Enabling MMU with identity mapping...Success
Transferring control to kernel...
```