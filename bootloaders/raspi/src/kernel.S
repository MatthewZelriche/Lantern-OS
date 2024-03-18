# xa is needed to prevent objcopy from stripping this out
# no need to mark this with any kind of label, we use the KERNEL_PHYS_* linker vars to locate
# the kernel
.section ".kernel", "xa"
.incbin "out/kernel"