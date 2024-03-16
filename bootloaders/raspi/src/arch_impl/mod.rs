use common::{
    allocators::page_frame_allocator::FrameAllocator, arch::Arch,
    memory::address_space::AddressSpace, util::error::AddressSpaceError,
};

use crate::paging::page_table::PageTable;

pub struct ArchImpl {}

impl Arch for ArchImpl {
    unsafe fn new_address_space<A: FrameAllocator>(
        translation: fn(usize) -> usize,
        frame_allocator: A,
    ) -> Result<impl AddressSpace, AddressSpaceError> {
        PageTable::new(translation, frame_allocator)
    }
}
