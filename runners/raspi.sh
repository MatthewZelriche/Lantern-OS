#!/bin/bash

mkdir -p out/
rust-objcopy $1 -O binary out/kernel8.img

if [ $2 = "raspi3" ] ; then
   qemu-system-aarch64 -serial stdio -M raspi3b -kernel out/kernel8.img -dtb bootloaders/raspi/vendor/bcm2710-rpi-3-b.dtb
elif [ $2 = "raspi4" ] ; then
   ${QEMU_PATH}qemu-system-aarch64 -serial stdio -M raspi4b -kernel out/kernel8.img -dtb bootloaders/raspi/vendor/bcm2711-rpi-4-b.dtb
else
   echo "Invalid raspi board version specified"
fi