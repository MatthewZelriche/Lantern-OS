use bitfield::BitRange;
use common::{
    allocators::page_frame_allocator::FrameAllocator,
    memory::{
        address_space::{AddressSpace, MemoryAttributes},
        PhysAddr,
    },
    read_linker_var,
    util::{error::AddressSpaceError, linker_variables::__PG_SIZE},
};
use core::slice::from_raw_parts_mut;
use tock_registers::{
    interfaces::{ReadWriteable, Readable},
    register_bitfields,
    registers::InMemoryRegister,
};

use super::memory_attribute::translate_memory_attrib;

const SIZE_4KIB: u64 = 4096;
const SIZE_2MIB: u64 = 2 * 1024 * 1024;
const SIZE_1GIB: u64 = 1024 * 1024 * 1024;

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

type TableDescriptor = InMemoryRegister<u64, TABLE::Register>;
type BlockDescriptor = InMemoryRegister<u64, BLOCK::Register>;
type PageDescriptor = InMemoryRegister<u64, PAGEENTRY4KIB::Register>;

pub struct PageTable<'a, A: FrameAllocator> {
    lvl0_table: &'a mut [u64],
    address_translation: fn(usize) -> usize,
    frame_allocator: A,
}

// TODO: Currently only supports 4KiB page granule
// TODO: Implement Drop
impl<'a, A: FrameAllocator> PageTable<'a, A> {
    // Unsafe because bad things will happen if the address translation function is not correct
    pub unsafe fn new(
        address_translation: fn(usize) -> usize,
        frame_allocator: A,
    ) -> Result<Self, AddressSpaceError> {
        if read_linker_var!(__PG_SIZE) != 4096 {
            return Err(AddressSpaceError);
        }

        let lvl0_table_phys_ptr = frame_allocator
            .allocate_zeroed_pages(1, address_translation)
            .map_err(|_| AddressSpaceError)? as *mut u64;
        let lvl0_table = from_raw_parts_mut(lvl0_table_phys_ptr, 4096 / 8);

        Ok(Self {
            lvl0_table,
            address_translation,
            frame_allocator,
        })
    }

    // Unsafe because bad things will happen if the address translation function is not correct
    pub unsafe fn set_translator(&mut self, address_translation: fn(usize) -> usize) {
        self.address_translation = address_translation;
    }

    pub fn virt_to_phys(&mut self, virt_addr: u64) -> Result<PhysAddr, AddressSpaceError> {
        // Get the lvl0 descriptor entry
        let lvl0_idx: u64 = virt_addr.bit_range(47, 39);
        let lvl0_descriptor = TableDescriptor::new(self.lvl0_table[lvl0_idx as usize]);
        if !lvl0_descriptor.is_set(TABLE::VALID) {
            return Err(AddressSpaceError);
        }

        let lvl1_table_ptr = (lvl0_descriptor.read(TABLE::NEXT_ADDR) << 12) as *mut u64;
        // Safe because a valid lvl 0 descriptor guaruntees a valid lvl1 table
        let lvl1_idx: u64 = virt_addr.bit_range(38, 30);
        let lvl1_table = unsafe { from_raw_parts_mut(lvl1_table_ptr, 4096 / 8) };
        let lvl1_entry = BlockDescriptor::new(lvl1_table[lvl1_idx as usize]);
        if !lvl1_entry.is_set(BLOCK::VALID) {
            return Err(AddressSpaceError);
        } else if !lvl1_entry.is_set(BLOCK::TABLE) {
            // Found a 1GIB page entry
            let block_phys_start = lvl1_entry.read(BLOCK::OUT_ADDR) << 12;
            let phys_lower_bits: u64 = virt_addr.bit_range(29, 0);
            return Ok((block_phys_start | phys_lower_bits).try_into().unwrap());
        }

        // Must be a table pointer to a lvl 2...
        todo!();
        return Err(AddressSpaceError);
    }

    pub fn map_1gib_page(
        &mut self,
        virt_start: u64,
        phys_start: u64,
        attr: MemoryAttributes,
    ) -> bool {
        if virt_start % SIZE_1GIB != 0 && phys_start % SIZE_1GIB != 0 {
            return false;
        }

        // Get the lvl0 descriptor entry
        let lvl0_idx: u64 = virt_start.bit_range(47, 39);
        let lvl0_descriptor = TableDescriptor::new(self.lvl0_table[lvl0_idx as usize]);
        if !lvl0_descriptor.is_set(TABLE::VALID) {
            let page_phys_addr = self
                .frame_allocator
                .allocate_zeroed_pages(1, self.address_translation)
                .unwrap() as u64;
            lvl0_descriptor.modify(TABLE::VALID::SET);
            lvl0_descriptor.modify(TABLE::TABLE::SET);
            lvl0_descriptor.modify(TABLE::NEXT_ADDR.val(page_phys_addr.bit_range(47, 12)));

            // Store the modified lvl0 entry back into the table
            self.lvl0_table[lvl0_idx as usize] = lvl0_descriptor.get();
        }

        let lvl1_table_ptr = (lvl0_descriptor.read(TABLE::NEXT_ADDR) << 12) as *mut u64;
        // Safe because a valid lvl 0 descriptor guaruntees a valid lvl1 table
        let lvl1_idx: u64 = virt_start.bit_range(38, 30);
        let lvl1_table = unsafe { from_raw_parts_mut(lvl1_table_ptr, 4096 / 8) };
        let lvl1_entry = BlockDescriptor::new(lvl1_table[lvl1_idx as usize]);
        if lvl1_entry.is_set(BLOCK::VALID) {
            panic!("Attempted to remap page in page table!");
        }
        lvl1_entry.modify(BLOCK::VALID::SET);
        lvl1_entry.modify(BLOCK::TABLE::CLEAR);
        lvl1_entry.modify(BLOCK::ATTR_IDX.val(translate_memory_attrib(attr) as u64));
        lvl1_entry.modify(BLOCK::OUT_ADDR.val(phys_start.bit_range(47, 30)));
        // Store the lvl1 entry back into the table
        lvl1_table[lvl1_idx as usize] = lvl1_entry.get();

        true
    }

    pub fn map_2mib_page(&mut self, virt_start: u64, phys_start: u64) -> bool {
        todo!()
    }

    pub fn map_4kib_page(&mut self, virt_start: u64, phys_start: u64) -> bool {
        todo!()
    }

    pub fn unmap_1gib_page(&mut self, virt_start: u64, phys_start: u64) -> bool {
        todo!()
    }

    pub fn unmap_2mib_page(&mut self, virt_start: u64, phys_start: u64) -> bool {
        todo!()
    }

    pub fn unmap_4kib_page(&mut self, virt_start: u64, phys_start: u64) -> bool {
        todo!()
    }
}

impl<'a, A: FrameAllocator> AddressSpace for PageTable<'a, A> {
    fn set_active(&mut self) -> bool {
        todo!()
    }

    fn map_range(
        &mut self,
        virt_start: usize,
        phys_start: usize,
        size: usize,
        attr: MemoryAttributes,
    ) -> bool {
        todo!()
    }

    fn unmap_range(&mut self, virt_start: usize, phys_start: usize, size: usize) -> bool {
        todo!()
    }

    fn translate(&mut self, virt_addr: usize) -> Result<PhysAddr, AddressSpaceError> {
        self.virt_to_phys(virt_addr.try_into().unwrap())
    }
}
