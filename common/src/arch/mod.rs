use crate::{
    allocators::page_frame_allocator::FrameAllocator, memory::address_space::AddressSpace,
    util::error::AddressSpaceError,
};

pub trait Arch {
    unsafe fn new_address_space<A: FrameAllocator>(
        translation: fn(usize) -> usize,
        frame_allocator: A,
    ) -> Result<impl AddressSpace, AddressSpaceError>;
}
