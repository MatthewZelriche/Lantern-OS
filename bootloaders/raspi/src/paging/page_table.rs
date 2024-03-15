use core::{
    alloc::Layout,
    mem::{size_of, ManuallyDrop},
    slice::from_raw_parts_mut,
};

use alloc::alloc::alloc_zeroed;
use common::{
    memory::address_space::AddressSpace,
    read_linker_var,
    util::{error::AddressSpaceError, linker_variables::__PG_SIZE},
};
use tock_registers::{register_bitfields, registers::InMemoryRegister};

register_bitfields!(
   u64,

   TABLE [
      VALID     OFFSET(0)  NUMBITS(1),
      TABLE     OFFSET(1)  NUMBITS(1),  // Must always be one!
      IGNORED   OFFSET(2)  NUMBITS(10),
      NEXT_ADDR OFFSET(12) NUMBITS(36),
      RES0      OFFSET(48) NUMBITS(4),
      IGNORED1  OFFSET(52) NUMBITS(7),
      PXNTABLE  OFFSET(59) NUMBITS(1),
      XNTABLE   OFFSET(60) NUMBITS(1),
      APTABLE   OFFSET(61) NUMBITS(2),
      NSTABLE   OFFSET(63) NUMBITS(1),
    ],

    BLOCK [
        VALID      OFFSET(0)  NUMBITS(1),
        TABLE      OFFSET(1)  NUMBITS(1), // Must always be zero!
        ATTR_IDX   OFFSET(2)  NUMBITS(2),
        NS         OFFSET(5)  NUMBITS(1),
        AP         OFFSET(6)  NUMBITS(2),
        SH         OFFSET(8)  NUMBITS(2),
        AF         OFFSET(10) NUMBITS(1),
        NG         OFFSET(11) NUMBITS(1),
        RES0       OFFSET(12) NUMBITS(18),
        OUT_ADDR   OFFSET(30) NUMBITS(18),
        RES0_1     OFFSET(48) NUMBITS(4),
        CONTIGUOUS OFFSET(52) NUMBITS(1),
        PXN        OFFSET(53) NUMBITS(1),
        UXN        OFFSET(54) NUMBITS(1),
        SOFTWARE   OFFSET(55) NUMBITS(4),
        IGNORED    OFFSET(59) NUMBITS(5),
      ],

    PAGEENTRY4KIB [
        VALID      OFFSET(0)  NUMBITS(1),
        RES1       OFFSET(1)  NUMBITS(1), // Must always be one!
        ATTR_IDX   OFFSET(2)  NUMBITS(2),
        NS         OFFSET(5)  NUMBITS(1),
        AP         OFFSET(6)  NUMBITS(2),
        SH         OFFSET(8)  NUMBITS(2),
        AF         OFFSET(10) NUMBITS(1),
        NG         OFFSET(11) NUMBITS(1),
        OUT_ADDR   OFFSET(12) NUMBITS(36),
        RES0       OFFSET(48) NUMBITS(4),
        CONTIGUOUS OFFSET(52) NUMBITS(1),
        PXN        OFFSET(53) NUMBITS(1),
        UXN        OFFSET(54) NUMBITS(1),
        SOFTWARE   OFFSET(55) NUMBITS(4),
        IGNORED    OFFSET(59) NUMBITS(5),
    ],
);

union Descriptor {
    table: ManuallyDrop<InMemoryRegister<u64, TABLE::Register>>,
    block: ManuallyDrop<InMemoryRegister<u64, BLOCK::Register>>,
    page_entry: ManuallyDrop<InMemoryRegister<u64, PAGEENTRY4KIB::Register>>,
}

pub struct PageTable<'a> {
    lvl1_table: &'a mut [Descriptor],
    address_translation: fn(usize) -> usize,
}

// TODO: Currently only supports 4KiB page granule
// TODO: Implement Drop
impl<'a> PageTable<'a> {
    // Unsafe because bad things will happen if the address translation function is not correct
    pub unsafe fn new(address_translation: fn(usize) -> usize) -> Result<Self, AddressSpaceError> {
        if read_linker_var!(__PG_SIZE) != 4096 {
            return Err(AddressSpaceError);
        }

        // SAFETY: Safe to alloc here because we are ensuring the correct size and alignment, and the memory
        // is owned exclusively by this page table. Safe to construct a slice since zeroed memory is a valid
        // bit representation for each table entry.
        let lvl1_table: &mut [Descriptor] = unsafe {
            let ptr =
                alloc_zeroed(Layout::from_size_align(4096, 4096).map_err(|_| AddressSpaceError)?)
                    as *mut Descriptor;
            from_raw_parts_mut(ptr, 4096 / size_of::<Descriptor>())
        };

        Ok(Self {
            lvl1_table,
            address_translation,
        })
    }

    // Unsafe because bad things will happen if the address translation function is not correct
    pub unsafe fn set_translator(&mut self, address_translation: fn(usize) -> usize) {
        self.address_translation = address_translation;
    }

    pub fn map_1gib_page(&mut self, virt_start: usize, phys_start: usize) -> bool {
        todo!()
    }

    pub fn map_2mib_page(&mut self, virt_start: usize, phys_start: usize) -> bool {
        todo!()
    }

    pub fn map_4kib_page(&mut self, virt_start: usize, phys_start: usize) -> bool {
        todo!()
    }

    pub fn unmap_1gib_page(&mut self, virt_start: usize, phys_start: usize) -> bool {
        todo!()
    }

    pub fn unmap_2mib_page(&mut self, virt_start: usize, phys_start: usize) -> bool {
        todo!()
    }

    pub fn unmap_4kib_page(&mut self, virt_start: usize, phys_start: usize) -> bool {
        todo!()
    }
}

impl<'a> AddressSpace for PageTable<'a> {
    fn set_active(&mut self) -> bool {
        todo!()
    }

    fn map_range(&mut self, virt_start: usize, phys_start: usize, size: usize) -> bool {
        todo!()
    }

    fn unmap_range(&mut self, virt_start: usize, phys_start: usize, size: usize) -> bool {
        todo!()
    }
}
