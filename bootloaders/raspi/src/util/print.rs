use crate::device_drivers::uart0::Pl011;
use common::{
    concurrency::single_threaded_lock::SingleThreadedLock,
    util::single_threaded_cell::SingleThreadedCell,
};

pub static UART0: SingleThreadedCell<SingleThreadedLock<Pl011>> = SingleThreadedCell::new();

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        writeln!(crate::util::print::UART0.get().unwrap().lock(), $($arg)*).unwrap();
    }};
}
