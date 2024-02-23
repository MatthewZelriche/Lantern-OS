use core::cell::UnsafeCell;

/// A cell for interior mutability in guarunteed single-threaded environments
///
/// This is essentially a lazily initialized Sync-enabled wrapper for UnsafeCell, for use in highly specific
/// environments such as bare-metal bootloaders with only one core and no interrupts.
///
/// # Safety
///
/// Using this struct outside of a guarunteed single-threaded environment (including with interrupts disabled)
/// is unsafe.
pub struct SingleThreadedCell<T>(UnsafeCell<Option<T>>);

impl<T> SingleThreadedCell<T> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    /// Sets the wrapped value
    pub fn set(&self, val: T) {
        let inner = unsafe { &mut *self.0.get() };
        *inner = Some(val);
    }

    /// Gets the wrapped value, if it exists
    pub fn get(&self) -> Option<&mut T> {
        unsafe { &mut *self.0.get() }.as_mut()
    }

    /// Destroys the wrapped value. Useful if you are about to enter a multithreaded environment and
    /// want to be sure this is never accessed again.
    pub fn clear(&self) {
        let inner = unsafe { &mut *self.0.get() };
        *inner = None;
    }
}

unsafe impl<T> Sync for SingleThreadedCell<T> {}
