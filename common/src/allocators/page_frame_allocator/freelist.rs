use super::PageFrameAllocator;

pub struct FreelistEntry(Option<*mut FreelistEntry>);

pub struct FreelistPFA {
    head: Option<*mut FreelistEntry>,
    free_count: usize,
}

impl FreelistPFA {
    pub const fn new() -> Self {
        Self {
            head: None,
            free_count: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.free_count
    }
}

impl PageFrameAllocator for FreelistPFA {
    fn allocate(&mut self) -> *mut u8 {
        // Get the next free page
        if let Some(old_head_ptr) = self.head {
            // Update the head with the next available frame in the freelist
            // Safety: The only way a frame could have made its way onto this freelist is if it was added
            // via a call to free(), and we ensure the start of a freed frame contains a valid FreelistEntry
            self.head = unsafe { (*old_head_ptr).0 };

            return old_head_ptr as *mut u8;
        } else {
            panic!("Physical page frame allocator ran out of memory to allocate!");
        }
    }

    unsafe fn free(&mut self, frame: *mut u8) {
        self.free_count += 1;

        let new_head_ptr = frame as *mut FreelistEntry;
        if let Some(old_head) = self.head {
            // Safety: We rely on the caller to ensure that the passed frame_ptr is indeed a frame
            // we can safely de-allocate, which means we can arbitrarily write to it. Additionally,
            // we require the pointer to be at the start of the page frame, for easy access to the freelist
            // bookkeeping nodes
            (*new_head_ptr).0 = Some(old_head);
        }

        self.head = Some(new_head_ptr);
    }
}
