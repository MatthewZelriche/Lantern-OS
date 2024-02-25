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
//!
//! GlobalWriter is necessary because it acts as essentially an UnsafeCell wrapper around something we know
//! is safe (our mutex implementation). This part is necessary because the core::fmt::Write trait requires
//! a mutable reference. So we retrieve the mutable reference through the UnsafeCell, knowing that data races
//! are prevented because MutexedWriter is guarunteed to prevent them.

use crate::{
    allocators::static_box::StaticBox, device_drivers::character_device::CharacterDevice,
    util::single_threaded_cell::SingleThreadedCell,
};
use core::cell::UnsafeCell;

pub static GLOBAL_WRITER: SingleThreadedCell<GlobalWriter> = SingleThreadedCell::new();

/// A CharacterDevice that is being internally protected by a mutex
///
/// This trait is necessary so that we can assign the static GlobalWriter variable at runtime without knowing
/// at compile time what Mutex implementation we plan to use. This allows us to set GlobalWriter to use
/// a SingleThreadedLock during early init, and a regular SpinLock after early init.
pub trait MutexedWriter: CharacterDevice + Send {}

/// Represents a thread-safe global writer for use with print, println, etc.
pub struct GlobalWriter(UnsafeCell<StaticBox<dyn MutexedWriter>>);

impl GlobalWriter {
    pub fn new(writer: StaticBox<dyn MutexedWriter>) -> Self {
        Self(UnsafeCell::new(writer))
    }

    /// Gets a mutable reference to the underlying MutexedWriter
    ///
    /// # Safety: Safe because we mandate that MutexedWriter MUST be wrapped in a valid mutex.
    /// This provides us the ability to access &mut methods on MutexedWriter, without GlobalWriter
    /// needing to know about the specific mutex implementation we chose.
    pub fn get(&self) -> &mut StaticBox<dyn MutexedWriter> {
        unsafe { &mut *self.0.get() }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            write!(print::GLOBAL_WRITER.get().unwrap().get(), $($arg)*).unwrap();
        }
    };
}
