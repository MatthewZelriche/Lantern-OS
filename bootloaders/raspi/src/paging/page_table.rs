use core::alloc::Layout;

use common::{
    allocators::page_frame_allocator::Allocator,
    memory::address_space::AddressSpace,
    read_linker_var,
    util::{error::AddressSpaceError, linker_variables::__PG_SIZE},
};

pub struct PageTable<'a> {
    lvl1_table: &'a mut [u8],
}

// TODO: Currently only supports 4KiB page granule
impl<'a> PageTable<'a> {
    pub fn new<T: Allocator>(allocator: &mut T) -> Result<Self, AddressSpaceError> {
        if read_linker_var!(__PG_SIZE) != 4096 {
            return Err(AddressSpaceError);
        }

        let lvl1_table = unsafe {
            allocator
                .allocate(Layout::from_size_align(4096, 4096).unwrap())
                .unwrap()
                .as_mut()
        };

        Ok(Self { lvl1_table })
    }
}

impl<'a> AddressSpace for PageTable<'a> {
    fn set_active(&mut self) -> bool {
        todo!()
    }
}
