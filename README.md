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