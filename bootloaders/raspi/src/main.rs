#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use common::{
    allocators::page_frame_allocator::bump::BumpPFA,
    read_linker_var,
    util::linker_variables::{__KERNEL_END, __PG_SIZE},
};
use core::{arch::global_asm, fmt::Write, panic::PanicInfo};
use device_drivers::{
    gpio::{Gpio, GPIO_PHYS_BASE},
    uart0::{Pl011, PL011_PHYS_BASE},
};

use crate::device_drivers::mailbox::{
    message::{SetClockRate, CLOCK_UART},
    Mailbox, MAILBOX_PHYS_BASE,
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
#[link_section = ".kernel"]
static KERNEL: &'static [u8] = include_bytes!("../../../out/kernel");

#[no_mangle]
pub extern "C" fn bootloader_main(dtb_ptr: *const u8) -> ! {
    // Set up a simple uart that we will use until we enable virtual memory mapping - to get some
    // meaningful output as early as possible.
    let mut uart = early_init_uart();
    writeln!(uart, "PL011 UART0 Device Driver initialized");

    // On raspi, we can safety assume at least 960 MiB (ignoring VRAM reserved memory & MMIO)
    // So we can just grab the first 20MiB after the kernel to allocate our page tables, it should be plenty
    let range_start = read_linker_var!(__KERNEL_END);
    let range_end = range_start + 0x1400000;
    let range_middle = (range_end + range_start) / 2;
    // Create two bumpPFAs. The first will allocate pages to the kernel page table and exist for
    // the static duration of the kernel's lifetime, so we will have to track its location.
    // The second PFA will only be to identity map memory so we can jump to the higher half, and
    // can be discarded after we jump.
    let page_size = read_linker_var!(__PG_SIZE);
    let mut saved_pfa =
        unsafe { BumpPFA::new(range_start, range_middle - page_size, page_size).unwrap() };
    let mut temp_pfa = unsafe { BumpPFA::new(range_middle, range_end, page_size).unwrap() };

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

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
