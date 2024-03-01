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
- [X] Implement safe global kprint macros for use in bootloader and kernel
- [X] Set up custom test framework
- [X] Implement physical page frame allocator
- [ ] Parse dtb and construct a memory map
- [ ] Enable the MMU to an identity mapping
- [ ] Implement synchronization primitives with MMU support (mutex, barrier)
- [ ] Initialize secondary cores
- [ ] Adjust virtual memory to load kernel in the higher half
- [ ] Framebuffer driver for printing to the screen 