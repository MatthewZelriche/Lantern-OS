//! Provides global, thread-safe access to a CharacterDevice designaed for use with print* macros
//!
//! The provided static variable, GLOBAL_WRITER, required careful implementation so that it's concrete
//! implementation could be swapped out at runtime while still remaining thread-safe as a global variable.
//! Firstly, it makes use of a SingleThreadedCell. Think of SingleThreadedCell as a OnceCell that you can
//! repeatedly re-assign to, but only while in a single-threaded context. After leaving the single-threaded
//! context, the SingleThreadedCell is to be read-only. This allows us to repeatedly assign a GlobalWriter
//! to the GLOBAL_WRITER static variable before entering a multi-threaded environment.
//!
//! Why is this necessary? Because GlobalWriter requires a different Mutex implementation depending on whether
//! the MMU is enabled or not. If the MMU is not enabled, we don't have access to hardware synchronization
//! primitives, and we can't use a regular Mutex. But we can't just use a SingleThreadedMutex, because
//! eventually the kernel will run in a multi-threaded environment.

use common::{
    allocators::static_box::StaticBox, device_drivers::character_device::CharacterDevice,
    util::single_threaded_cell::SingleThreadedCell,
};
use core::{cell::UnsafeCell, ops::DerefMut};

pub static GLOBAL_WRITER: SingleThreadedCell<GlobalWriter> = SingleThreadedCell::new();

/// Essentially a carbon copy of lock_api's RawMutex, but without the const associated variable. We needed
/// to strip that so that we can make it object safe for dyn.
pub trait RawWriterMutex: Send {
    fn lock(&self);
    fn try_lock(&self) -> bool;
    unsafe fn unlock(&self);
}

/// Represents a thread-safe global writer for use with print, println, etc.
///
/// Internally, GlobalWriter uses a dynamic dispatch for two things: First, to erase hardware-specific
/// details about the given CharacterDevice, and Second, to allow runtime swapping of the mutex we use
/// to ensure there exists only one mutable reference to CharacterDevice at a time.
pub struct GlobalWriter {
    mutex: StaticBox<dyn RawWriterMutex>,
    writer: UnsafeCell<StaticBox<dyn CharacterDevice>>,
}

impl GlobalWriter {
    pub fn new(
        writer: StaticBox<dyn CharacterDevice>,
        mutex: StaticBox<dyn RawWriterMutex>,
    ) -> Self {
        Self {
            mutex,
            writer: UnsafeCell::new(writer),
        }
    }

    /// Locks the wrapped Character Device, returning an exclusive mutable reference for use in a closure.
    ///
    /// Beware of triggering deadlocks through the use of the closure. The caller should make their closure
    /// as simple as possible to avoid deadlocks. The mutex that protects CharacterDevice is not released
    /// until after the caller's closure returns successfully.
    pub fn lock<F: FnOnce(&mut dyn CharacterDevice)>(&self, closure: F) {
        self.mutex.lock();

        // Safety: Safe because we enforce with the mutex that there will only ever be one mutable reference
        // to the wrapped CharacterDevice. We can also safely know that there are no concurrent non-mutable
        // references, because there is no way to externally access a non-mutable reference to CharacterDevice.
        let writer = unsafe { &mut *self.writer.get() };
        closure(writer.deref_mut());

        unsafe {
            // Safety: Every unlock must be paired with a successfull lock. We see trivially that we can't reach
            // this point without a successful lock, and we know that the supplied closure can't unlock the
            // mutex before we reach this point, because the mutex is only accessible from within GlobalWriter.
            self.mutex.unlock();
        }
    }
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {{
        crate::print::GLOBAL_WRITER
            .get()
            .unwrap()
            .lock(|writer| write!(writer, $($arg)*).unwrap());
    }};
}

#[macro_export]
macro_rules! kprintln {
    ($($arg:tt)*) => {{
        crate::print::GLOBAL_WRITER
            .get()
            .unwrap()
            .lock(|writer| writeln!(writer, $($arg)*).unwrap());
    }};
}
