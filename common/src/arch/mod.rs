use crate::{
    allocators::page_frame_allocator::Allocator, memory::address_space::AddressSpace,
    util::error::AddressSpaceError,
};

pub trait Arch {
    fn new_address_space<T: Allocator>(
        allocator: &mut T,
    ) -> Result<impl AddressSpace, AddressSpaceError>;
}
