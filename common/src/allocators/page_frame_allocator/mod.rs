pub mod bump;
pub mod freelist;

pub trait PageFrameAllocator {
    fn allocate(&mut self) -> *mut u8;
    unsafe fn free(&mut self, frame: *mut u8);
}
