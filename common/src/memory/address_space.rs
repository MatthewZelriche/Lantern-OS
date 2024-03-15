pub trait AddressSpace {
    fn set_active(&mut self) -> bool;
    fn map_range(&mut self, virt_start: usize, phys_start: usize, size: usize) -> bool;
    fn unmap_range(&mut self, virt_start: usize, phys_start: usize, size: usize) -> bool;
}
