use super::Allocator;
use crate::util::error::AllocError;
use core::ptr::{slice_from_raw_parts_mut, NonNull};

pub struct BumpPFA {
    start_frame: usize,
    end_frame: usize,
    next: usize,
    page_size: usize,
}

impl BumpPFA {
    // TODO: Proper documentation. end_frame is EXCLUSIVE
    pub unsafe fn new(start_frame: usize, end_frame: usize, page_size: usize) -> Result<Self, ()> {
        if start_frame % page_size != 0 || end_frame % page_size != 0 {
            return Err(());
        }
        if end_frame < start_frame {
            return Err(());
        }

        Ok(Self {
            start_frame,
            end_frame,
            next: start_frame,
            page_size,
        })
    }

    pub unsafe fn allocated_range(&self) -> (usize, usize) {
        (self.start_frame, self.next)
    }

    pub fn allocate_contiguous_pages(&mut self, num_pages: usize) -> Result<*mut [u8], AllocError> {
        if num_pages == 0 {
            return Err(AllocError);
        }
        if self.next + ((num_pages - 1) * self.page_size) >= self.end_frame {
            return Err(AllocError);
        }
        let page = self.next;
        self.next += self.page_size * num_pages;
        Ok(slice_from_raw_parts_mut(
            page as *mut u8,
            self.page_size * num_pages,
        ))
    }
}

impl Allocator for BumpPFA {
    fn allocate(
        &mut self,
        layout: core::alloc::Layout,
    ) -> Result<core::ptr::NonNull<[u8]>, AllocError> {
        if layout.align() > self.page_size {
            return Err(AllocError);
        }

        let num_pages = layout.size().div_ceil(self.page_size);
        NonNull::new(self.allocate_contiguous_pages(num_pages)?).ok_or(AllocError)
    }

    unsafe fn deallocate(&mut self, _: core::ptr::NonNull<u8>, _: core::alloc::Layout) {
        panic!("Attempted to free an individual bump-allocated page!");
    }
}
