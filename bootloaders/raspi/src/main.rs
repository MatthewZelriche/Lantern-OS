#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use common::{
    memory::{memory_size::MemorySize, page_frame_allocator::PageFrameAllocator},
    read_linker_var,
    util::linker_variables::__PG_SIZE,
};
use core::{arch::global_asm, fmt::Write};
use device_drivers::{
    gpio::{Gpio, GPIO_PHYS_BASE},
    uart0::{Pl011, PL011_PHYS_BASE},
};

use crate::{
    device_drivers::mailbox::{
        message::{SetClockRate, CLOCK_UART},
        Mailbox, MAILBOX_PHYS_BASE,
    },
    device_tree::RaspiDeviceTree,
};

mod device_drivers;
mod device_tree;
#[cfg(test)]
mod test;
mod writer_mutexes;

#[cfg(feature = "raspi3")]
static RASPI_VERSION: u8 = 3;
#[cfg(feature = "raspi4")]
static RASPI_VERSION: u8 = 4;

global_asm!(include_str!("main.S"));

#[no_mangle]
pub extern "C" fn bootloader_main(dtb_ptr: *const u8) -> ! {
    // Set up a simple uart that we will use until we enable virtual memory mapping - to get some
    // meaningful output as early as possible.
    let mut uart = early_init_uart();
    writeln!(uart, "PL011 UART0 Device Driver initialized");

    // Before we can enable virtual memory mapping, we need some way of dynamically allocating memory for
    // the page tables. So our next order of business is to initialize a physical page frame allocator
    // But wait! Without a memory map, we don't know what regions of memory are safe to add to the allocator.
    // Why don't we have a memory map? Because a memory map requires dynamic allocation due to the variable
    // number of entries.
    // Thankfully, on the raspberry pi, we always know that the first frame is reserved, and that all
    // subsequent frames up to 0x80000 (kernel start) are guarunteed to be free. With 4KiB pages, this gives
    // us just under half a MiB to add to the page frame allocator.
    // TODO: We can't support 1MiB pages on the raspi with this assumption
    let mut frame_alloc = PageFrameAllocator::new();
    let page_size = read_linker_var!(__PG_SIZE);
    for page in (0x1000_usize.next_multiple_of(page_size)..0x80000).step_by(page_size) {
        unsafe {
            // Safety: As above, on raspi we are guarunteed for all pages before 0x80000 except the first one
            // be free of any important data
            frame_alloc.free(page as *mut u8);
        }
    }
    writeln!(
        uart,
        "Added {} page frames to the freelist",
        frame_alloc.len()
    );

    // Parse the dtb for total physical memory ranges
    let dt = RaspiDeviceTree::new(dtb_ptr).expect("Failed to read device tree! Error");
    dt.for_each_memory(|addr, size| {
        // For now we just print them to UART
        writeln!(
            uart,
            "Found RAM block from raspi{} DTB: Addr {:#010X}, Size {:05} MiB",
            RASPI_VERSION,
            addr,
            MemorySize::new(size as usize).as_mebibytes()
        );
    });

    // TODO: Before we can proceed past this point, we need to set up the GLOBAL_WRITER.
    loop {}
    #[cfg(test)]
    test_main();
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
