use aarch64_cpu::{
    asm::barrier,
    registers::{Writeable, MAIR_EL1, SCTLR_EL1, TCR_EL1, TTBR0_EL1, TTBR1_EL1},
};
use common::allocators::page_frame_allocator::FrameAllocator;

use super::page_table::PageTable;

pub unsafe fn enable_mmu<A: FrameAllocator>(ttbr0: &mut PageTable<A>, ttbr1: &mut PageTable<A>) {
    // idx 0: Strongly Ordered Device memory
    MAIR_EL1.write(MAIR_EL1::Attr0_Device::nonGathering_nonReordering_noEarlyWriteAck);

    // Set base addr for page tables
    TTBR0_EL1.set_baddr(ttbr0.as_raw() as u64);
    TTBR1_EL1.set_baddr(ttbr1.as_raw() as u64);

    // Minimum value for T0SZ and T1SZ is 16, splitting entire 48-bit virtual address space
    // between user and kernel mode.
    // Also set page granule to 4KiB for both page tables
    TCR_EL1.write(
        TCR_EL1::IPS::Bits_48
            + TCR_EL1::T0SZ.val(16)
            + TCR_EL1::T1SZ.val(16)
            + TCR_EL1::TG0::KiB_4
            + TCR_EL1::TG1::KiB_4,
    );
    barrier::isb(barrier::SY);
    SCTLR_EL1.write(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);
    barrier::isb(barrier::SY);
}
