use crate::{allocators::StaticAlloc, util::error::AllocError};
use core::{
    alloc::Layout,
    marker::Unsize,
    ops::{CoerceUnsized, Deref, DerefMut},
    ptr::NonNull,
};

/// A smart pointer that holds exclusive ownership over data that will live statically (until the end of the program)
///
/// StaticBox allocates its memory via the StaticAlloc allocator trait. This means that once its memory is
/// allocated, that memory can never be freed. Appropriately, this means StaticBox does not implement Drop.
/// If you somehow let your StaticBox go out of scope, it means you've just implicitly leaked memory.
///
/// Generally used for objects that, once allocated, should exist for the duration of the program's lifetime.
/// It's benefit over other solutions such as static variables is that memory need not be allocated for the
/// object at compile-time.
pub struct StaticBox<T: ?Sized>(NonNull<T>);

impl<T> StaticBox<T> {
    pub fn new<A: StaticAlloc>(val: T, allocator: &mut A) -> Result<Self, AllocError> {
        let layout = Layout::new::<T>();
        let this_ptr = allocator.allocate_bytes(layout)?.as_ptr() as *mut T;
        unsafe {
            this_ptr.write(val);
        }

        Ok(Self(NonNull::new(this_ptr).ok_or(AllocError)?))
    }
}

impl<T: ?Sized> Deref for StaticBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for StaticBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<StaticBox<U>> for StaticBox<T> {}

// Safety: StaticBox owns its data exclusively, so this type inherits T's Send status
unsafe impl<T: ?Sized + Send> Send for StaticBox<T> {}
