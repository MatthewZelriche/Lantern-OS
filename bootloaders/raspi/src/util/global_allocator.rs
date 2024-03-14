extern crate alloc;

use common::{
    allocators::page_frame_allocator::bump::BumpPFA,
    concurrency::single_threaded_lock::SingleThreadedLock,
    read_linker_var,
    util::{linker_variables::__PG_SIZE, single_threaded_cell::SingleThreadedCell},
};
use core::{alloc::GlobalAlloc, ptr::null_mut};

#[global_allocator]
pub static PAGE_FRAME_ALLOCATOR: PageFrameAllocator = PageFrameAllocator::new();
pub struct PageFrameAllocator(SingleThreadedCell<SingleThreadedLock<BumpPFA>>);

impl PageFrameAllocator {
    pub const fn new() -> Self {
        Self {
            0: SingleThreadedCell::new(),
        }
    }

    pub unsafe fn set(&self, inner: BumpPFA) {
        self.0.set(SingleThreadedLock::new(inner));
    }
}

// Safety: We are using interior mutability for this global allocator, but we are ensuring that the bootloader
// operates exclusively in a single threaded environment without interrupts, so our usage of SingleThreadedLock
// to implement interior mutability is safe.
unsafe impl GlobalAlloc for PageFrameAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let page_size = read_linker_var!(__PG_SIZE);
        let num_pages = layout.size().div_ceil(page_size);
        let pages = self
            .0
            .get()
            .unwrap()
            .lock()
            .allocate_contiguous_pages(num_pages);

        match pages {
            Ok(pages) => return pages as *mut u8,
            Err(_) => return null_mut(),
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {
        panic!("Attempted to dealloc on Bootloader Global Allocator!");
    }
}
