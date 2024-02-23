use crate::{
    device_drivers::{
        gpio::{Gpio, GPIO_PHYS_BASE},
        mailbox::{Mailbox, MAILBOX_PHYS_BASE},
        uart0::{Pl011, PL011_PHYS_BASE},
    },
    test::single_threaded_cell::SingleThreadedCell,
};
use core::fmt::Write;

pub mod single_threaded_cell;

pub static UART: SingleThreadedCell<Pl011> = SingleThreadedCell::new();

pub fn test_runner(tests: &[&dyn Fn()]) {
    let mut gpio: Gpio;
    let mut mailbox: Mailbox;
    let mut uart: Pl011;
    unsafe {
        gpio = Gpio::new(GPIO_PHYS_BASE);
        mailbox = Mailbox::new(MAILBOX_PHYS_BASE);
        uart = Pl011::new(PL011_PHYS_BASE, &mut gpio);
    }
    UART.set(uart);

    writeln!(
        UART.get().unwrap(),
        "TESTING BEGIN: RUN {} TESTS",
        tests.len()
    );

    for test in tests {
        test();
    }

    writeln!(UART.get().unwrap(), "TESTING END\n");
    UART.clear();
}
