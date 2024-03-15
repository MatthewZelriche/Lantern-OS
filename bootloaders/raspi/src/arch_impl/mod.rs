use common::{arch::Arch, memory::address_space::AddressSpace, util::error::AddressSpaceError};

use crate::paging::page_table::PageTable;

pub struct ArchImpl {}

impl Arch for ArchImpl {
    unsafe fn new_address_space(
        translation: fn(usize) -> usize,
    ) -> Result<impl AddressSpace, AddressSpaceError> {
        PageTable::new(translation)
    }
}
