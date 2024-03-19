use super::FrameAllocator;
use crate::{
    concurrency::single_threaded_lock::SingleThreadedLock, memory::PhysAddr,
    util::error::AllocError,
};
use core::slice::from_raw_parts_mut;

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

    pub fn allocated_range(&self) -> (usize, usize) {
        (self.start_frame, self.next)
    }

    pub fn allocate_contiguous_pages(&mut self, num_pages: usize) -> Result<PhysAddr, AllocError> {
        if num_pages == 0 {
            return Err(AllocError);
        }
        if self.next + ((num_pages - 1) * self.page_size) >= self.end_frame {
            return Err(AllocError);
        }
        let page = self.next;
        self.next += self.page_size * num_pages;
        Ok(page)
    }
}

pub struct SingleThreadedBumpPFA(SingleThreadedLock<BumpPFA>);

impl SingleThreadedBumpPFA {
    pub fn new(inner: SingleThreadedLock<BumpPFA>) -> Self {
        Self(inner)
    }

    pub fn allocated_range(&self) -> (usize, usize) {
        self.0.lock().allocated_range()
    }
}

unsafe impl<'a> FrameAllocator for &'a SingleThreadedBumpPFA {
    fn allocate_pages(&self, num_contiguous_pages: usize) -> Result<PhysAddr, AllocError> {
        self.0
            .lock()
            .allocate_contiguous_pages(num_contiguous_pages)
    }

    fn allocate_zeroed_pages(
        &self,
        num_contiguous_pages: usize,
        translation: fn(usize) -> usize,
    ) -> Result<PhysAddr, AllocError> {
        let mut inner = self.0.lock();
        let start_phys = inner.allocate_contiguous_pages(num_contiguous_pages)?;
        let start_virt = translation(start_phys);

        let slice = unsafe {
            from_raw_parts_mut(
                start_virt as *mut u8,
                num_contiguous_pages * inner.page_size,
            )
        };

        for byte in slice {
            *byte = 0;
        }
        Ok(start_phys)
    }

    unsafe fn deallocate_pages(&self, _: PhysAddr, _: usize) {
        panic!("Attempted to free bump-allocated pages!");
    }
}
