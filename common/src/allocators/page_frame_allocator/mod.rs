use crate::{memory::PhysAddr, util::error::AllocError};

pub mod bump;
pub mod freelist;

pub unsafe trait FrameAllocator {
    fn allocate_pages(&self, num_contiguous_pages: usize) -> Result<PhysAddr, AllocError>;
    unsafe fn deallocate_pages(&self, addr: PhysAddr, num_contiguous_pages: usize);
}
