use crate::{memory::address_space::AddressSpace, util::error::AddressSpaceError};

pub trait Arch {
    unsafe fn new_address_space(
        translation: fn(usize) -> usize,
    ) -> Result<impl AddressSpace, AddressSpaceError>;
}
