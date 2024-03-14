use core::{alloc::Layout, slice::from_raw_parts_mut};

use alloc::alloc::alloc_zeroed;
use common::{
    memory::address_space::AddressSpace,
    read_linker_var,
    util::{error::AddressSpaceError, linker_variables::__PG_SIZE},
};

pub struct PageTable<'a> {
    lvl1_table: &'a mut [u8],
}

// TODO: Currently only supports 4KiB page granule
// TODO: Implement Drop
impl<'a> PageTable<'a> {
    pub fn new() -> Result<Self, AddressSpaceError> {
        if read_linker_var!(__PG_SIZE) != 4096 {
            return Err(AddressSpaceError);
        }

        // SAFETY: Safe to alloc here because we are ensuring the correct size and alignment, and the memory
        // is owned exclusively by this page table. Safe to construct a slice since zeroed memory is a valid
        // bit representation for each table entry.
        let lvl1_table = unsafe {
            let ptr =
                alloc_zeroed(Layout::from_size_align(4096, 4096).map_err(|_| AddressSpaceError)?);
            from_raw_parts_mut(ptr, 4096)
        };

        Ok(Self { lvl1_table })
    }
}

impl<'a> AddressSpace for PageTable<'a> {
    fn set_active(&mut self) -> bool {
        todo!()
    }
}
