use common::{
    allocators::page_frame_allocator::Allocator, arch::Arch, memory::address_space::AddressSpace,
    util::error::AddressSpaceError,
};

pub struct ArchImpl {}

impl Arch for ArchImpl {
    fn new_address_space<T: Allocator>(
        allocator: &mut T,
    ) -> Result<impl AddressSpace, AddressSpaceError> {
        todo!()
    }
}
