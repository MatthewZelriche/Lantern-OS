use core::cell::UnsafeCell;

/// A cell for interior mutability in guarunteed single-threaded environments
///
/// Think of this struct as a OnceCell, except you can re-assign it as many times as you want, but
/// only in a single-threaded environment. Once you've left the single-threaded environment, it is unsafe
/// to write to the SingleThreadedCell again. From there on out, it should be treated as strictly read-only
/// through the get method.
pub struct SingleThreadedCell<T>(UnsafeCell<Option<T>>);

impl<T> SingleThreadedCell<T> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    /// Sets the wrapped value
    ///
    /// # Safety
    ///
    /// It is NEVER safe to call this function in a multi-threaded environment! You can only ever call this
    /// method safely if you can guaruntee it is being called in a single-threaded environment. This includes
    /// disabled interrupts!
    pub unsafe fn set(&self, val: T) {
        let inner = &mut *self.0.get();
        *inner = Some(val);
    }

    /// Gets the wrapped value, if it exists
    pub fn get(&self) -> Option<&T> {
        // Safety: Safe because we enforced that mutable aliases to this data can only occur in a strictly
        // single-threaded environment (through calls to the set method)
        unsafe { &*self.0.get() }.as_ref()
    }
}

// Safety: Safe because we require the user to ensure that SingleThreadedCell can never be written to
// in a multi-threaded environment
unsafe impl<T: Send> Sync for SingleThreadedCell<T> {}
