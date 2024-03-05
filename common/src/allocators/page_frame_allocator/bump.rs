use super::PageFrameAllocator;

pub struct BumpPFA {
    start_frame: usize,
    end_frame: usize,
    next: usize,
    page_size: usize,
}

impl BumpPFA {
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
}

impl PageFrameAllocator for BumpPFA {
    fn allocate(&mut self) -> *mut u8 {
        if self.next > self.end_frame {
            panic!("Physical page frame allocator ran out of memory to allocate!");
        }
        let page = self.next;
        self.next += self.page_size;
        page as *mut u8
    }

    unsafe fn free(&mut self, _: *mut u8) {
        panic!("Attempted to free an individual bump-allocated page!");
    }
}
