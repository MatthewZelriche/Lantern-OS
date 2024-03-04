use crate::{allocators::StaticAlloc, util::error::AllocError};
use core::{alloc::Layout, ptr::NonNull};

pub struct StaticBumpAlloc {
    next: usize,
    mem_start: usize,
    mem_size: usize,
}

impl StaticBumpAlloc {
    pub unsafe fn new(start_addr: usize, size: usize) -> Self {
        Self {
            next: start_addr,
            mem_start: start_addr,
            mem_size: size,
        }
    }

    pub fn remaining(&self) -> usize {
        self.mem_size - self.next
    }
}

unsafe impl StaticAlloc for StaticBumpAlloc {
    fn allocate_bytes(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // Make sure the start of this memory block is properly aligned
        let start = self.next.next_multiple_of(layout.align());
        // Do we have enough remaining memory in this bump allocator?
        let new_next = start + layout.size();
        if new_next > self.mem_start + self.mem_size {
            // We've ran out of memory!
            return Err(AllocError);
        }
        self.next = new_next;

        Ok(NonNull::slice_from_raw_parts(
            NonNull::new(start as *mut u8).unwrap(),
            layout.size(),
        ))
    }
}
