use crate::util::error::AllocError;
use core::{alloc::Layout, ptr::NonNull};

pub mod bump;
pub mod freelist;

pub trait Allocator {
    fn allocate(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>;
    unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout);
}
