use core::ops::Deref;

/// A wrapper around a block of MMIO registers in memory.
///
/// This helper struct is necessary because we can't specify offsets in tock-registers structs with anything
/// except numerical literals. For example, GPIO_BASE + 0x10 would not compile. So instead we wrap tock-registers
/// structs into this wrapper that dereferences into the correct location in MMIO on demand.
pub struct RegisterRef<T> {
    start_addr: *mut T,
}

unsafe impl<T: Send> Send for RegisterRef<T> {}

impl<T> RegisterRef<T> {
    /// Wraps a tock-registers struct into a new RegisterRef
    ///
    /// # Safety
    /// start_addr must be the start of the MMIO memory that T represents.
    pub unsafe fn new(start_addr: usize) -> Self {
        Self {
            start_addr: start_addr as *mut T,
        }
    }
}

impl<T> Deref for RegisterRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.start_addr }
    }
}
