use core::fmt::Write;

use crate::device_drivers::uart0::Pl011;
use kernel::{
    concurrency::single_threaded_lock::SingleThreadedLock,
    device_drivers::character_device::CharacterDevice, print::MutexedWriter,
};

/// An implementation of MutexedWriter suitable for strictly single-threaded contexts.
///
/// Uses the SingleThreadedLock, which doesn't actually implement any locking whatsoever. As a result,
/// using this struct is safe only in environments where it can be guarunteed that there is only one
/// thread of execution. This is necessary because the ARM processor cannot access atomic instructions
/// until the MMU is enabled.
///
/// # Safety: It is unsafe to use this in any context that is not strictly single-threaded.
pub struct SingleThreadedMutexedWriter(SingleThreadedLock<Pl011>);

impl SingleThreadedMutexedWriter {
    pub fn new(uart: Pl011) -> Self {
        Self(SingleThreadedLock::new(uart))
    }
}

impl Write for SingleThreadedMutexedWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.lock().write_str(s)
    }
}

impl CharacterDevice for SingleThreadedMutexedWriter {
    fn write(&mut self, data: &[u8]) -> Result<usize, kernel::util::error::DeviceError> {
        self.0.lock().write(data)
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, kernel::util::error::DeviceError> {
        self.0.lock().read(buf)
    }
}

impl MutexedWriter for SingleThreadedMutexedWriter {}
