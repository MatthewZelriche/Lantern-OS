use super::PhysAddr;
use crate::util::error::AddressSpaceError;

pub enum MemoryAttributes {
    DeviceStronglyOrdered,
}

pub trait AddressSpace {
    fn set_active(&mut self) -> bool;
    fn map_range(
        &mut self,
        virt_start: usize,
        phys_start: usize,
        size: usize,
        attr: MemoryAttributes,
    ) -> bool;
    fn unmap_range(&mut self, virt_start: usize, phys_start: usize, size: usize) -> bool;
    fn translate(&mut self, virt_addr: usize) -> Result<PhysAddr, AddressSpaceError>;
}
