#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use common::{
    allocators::page_frame_allocator::bump::BumpPFA,
    concurrency::single_threaded_lock::SingleThreadedLock,
    read_linker_var,
    util::linker_variables::{__KERNEL_END, __PG_SIZE},
};
use core::arch::global_asm;
use device_drivers::{
    gpio::{Gpio, GPIO_PHYS_BASE},
    uart0::{Pl011, PL011_PHYS_BASE},
};

use crate::{
    device_drivers::mailbox::{
        message::{SetClockRate, CLOCK_UART},
        Mailbox, MAILBOX_PHYS_BASE,
    },
    paging::page_table::PageTable,
    util::global_allocator::PAGE_FRAME_ALLOCATOR,
};

mod arch_impl;
mod device_drivers;
mod device_tree;
pub mod paging;
#[cfg(test)]
mod test;
mod util;
mod writer_mutexes;

global_asm!(include_str!("main.S"));
#[link_section = ".kernel"]
static KERNEL: &'static [u8] = include_bytes!("../../../out/kernel");

#[no_mangle]
pub extern "C" fn bootloader_main(dtb_ptr: *const u8) -> ! {
    // Set up a simple uart that we will use until we enable virtual memory mapping - to get some
    // meaningful output as early as possible.
    let uart = early_init_uart();
    unsafe {
        util::print::UART0.set(SingleThreadedLock::new(uart));
    }
    println!("PL011 UART0 Device Driver initialized");

    // Initialize the page frame allocator for the bootloader
    // The Kernel will build and use its own page frame allocator. A BumpPFA was chosen for the bootloader,
    // which means calls to deallocate on the global allocator will panic. Normally, this would mean we can't
    // deallocate any pages we allocate in the bootloader. However, in this specific instance we will use a
    // small trick to allocate the pages for a "temporary" page table first, then record the position of the
    // BumpPFA after we have allocated this page table. This will tell us what region of the BumpPFA that we
    // can add to the memory map as safely reclaimable by the kernel.
    let kernel_end = read_linker_var!(__KERNEL_END);
    let page_size = read_linker_var!(__PG_SIZE);
    unsafe {
        // Safety: safe to call set(), because we are in a gaurunteed single threaded environment. Safe to
        // construct the BumpPFA because we know that range of memory is unused on the raspi.
        PAGE_FRAME_ALLOCATOR
            .set(BumpPFA::new(kernel_end, kernel_end + 0x1400000, page_size).unwrap())
    }
    println!(
        "Allocated free frames for bootloader in range {:#X} - {:#X}",
        kernel_end,
        kernel_end + 0x1400000
    );
    let identity_mapped_table = PageTable::new().unwrap();

    loop {}
}

fn early_init_uart() -> Pl011 {
    // Create temp device drivers needed to init the uart
    let mut gpio: Gpio;
    let mut mailbox: Mailbox;
    let uart: Pl011;
    unsafe {
        // Safety: The MMIO addresses are correct for the given Raspberry Pi board revision.
        gpio = Gpio::new(GPIO_PHYS_BASE);
        mailbox = Mailbox::new(MAILBOX_PHYS_BASE);
    }
    // Set the UART frequency to a known value & construct uart driver
    let mut uart_rate_msg = SetClockRate::new(CLOCK_UART, 30000000);
    mailbox.send_property_mail(&mut uart_rate_msg).unwrap();
    unsafe {
        // Safety: The MMIO address is correct and we have set the correct UART clock frequency
        uart = Pl011::new(PL011_PHYS_BASE, &mut gpio);
    }

    uart
}
