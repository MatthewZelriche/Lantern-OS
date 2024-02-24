use crate::{
    device_drivers::{
        gpio::{Gpio, GPIO_PHYS_BASE},
        mailbox::{Mailbox, MAILBOX_PHYS_BASE},
        uart0::{Pl011, PL011_PHYS_BASE},
    },
    test::single_threaded_cell::SingleThreadedCell,
};
use core::fmt::Write;

pub fn test_runner(tests: &[&dyn Fn()]) {
    // TODO

    for test in tests {
        test();
    }
}
