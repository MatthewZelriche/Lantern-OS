use core::{alloc::Layout, ptr::NonNull};

use crate::util::error::AllocError;

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

    pub fn allocate<T>(&mut self, val: T) -> Result<&'static mut T, AllocError> {
        let layout = Layout::new::<T>();
        self.allocate_bytes(layout).map(|x| unsafe {
            (x.as_ptr() as *mut T).write(val);
            &mut *(x.as_ptr() as *mut T)
        })
    }
}

unsafe trait StaticAlloc {
    fn allocate_bytes(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>;
}

unsafe impl StaticAlloc for StaticBumpAlloc {
    fn allocate_bytes(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let old_start = self.next;
        let start = self.next.next_multiple_of(layout.align());

        let res = Ok(NonNull::slice_from_raw_parts(
            NonNull::new(start as *mut u8).unwrap(),
            layout.size(),
        ));

        self.next = start + layout.size();
        res
    }
}
