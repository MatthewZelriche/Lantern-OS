use common::memory::address_space::MemoryAttributes;

pub fn translate_memory_attrib(attr: MemoryAttributes) -> u8 {
    match attr {
        MemoryAttributes::DeviceStronglyOrdered => 0,
    }
}
