#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use common::{
    allocators::{
        page_frame_allocator::{
            bump::{BumpPFA, SingleThreadedBumpPFA},
            FrameAllocator,
        },
        static_bump::StaticBumpAlloc,
    },
    concurrency::single_threaded_lock::SingleThreadedLock,
    memory::{
        address_space::MemoryAttributes,
        memory_map::{MemoryMap, MemoryMapEntry, MemoryMapType},
        memory_size::MemorySize,
    },
    read_linker_var,
    util::linker_variables::{__KERNEL_VIRT_START, __PG_SIZE},
};
use core::arch::global_asm;
use device_drivers::{
    gpio::{Gpio, GPIO_PHYS_BASE},
    uart0::{Pl011, PL011_PHYS_BASE},
};

use crate::{
    device_drivers::mailbox::{
        message::{SetClockRate, CLOCK_UART},
        Mailbox, MAILBOX_PHYS_BASE,
    },
    device_tree::RaspiDeviceTree,
    paging::{mmu::enable_mmu, page_table::PageTable},
};

mod arch_impl;
mod device_drivers;
mod device_tree;
pub mod paging;
#[cfg(test)]
mod test;
mod util;
mod writer_mutexes;

global_asm!(include_str!("main.S"));
global_asm!(include_str!("kernel.S"));

extern "C" {
    pub static __STACK_END: u8;
    pub static __STACK_START: u8;
    pub static __BOOTLOADER_START: u8;
    pub static __BOOTLOADER_END: u8;
    pub static __KERNEL_PHYS_START: u8;
    pub static __KERNEL_PHYS_END: u8;
}

#[no_mangle]
pub extern "C" fn bootloader_main(dtb_ptr: *const u8) -> ! {
    // Set up a simple uart that we will use until we enable virtual memory mapping - to get some
    // meaningful output as early as possible.
    let uart = early_init_uart();
    unsafe {
        util::print::UART0.set(SingleThreadedLock::new(uart));
    }
    println!("PL011 UART0 Device Driver initialized");

    // Create two bump allocators, one for temporary allocations that will be freed later, and one for
    // permanent allocations that will never be freed (eg kernel page table)
    let page_size = read_linker_var!(__PG_SIZE);
    let kernel_end = read_linker_var!(__KERNEL_PHYS_END);
    let second_alloc_start = kernel_end + 0x500000;
    let second_alloc_end = second_alloc_start + 0x500000;
    // SAFETY: Memory range for both allocators is guarunteed to be free, and we are guarunteed to be
    // in a single threaded environment during bootloading
    let pfa = unsafe {
        SingleThreadedBumpPFA::new(SingleThreadedLock::new(
            BumpPFA::new(kernel_end, second_alloc_start, page_size).unwrap(),
        ))
    };
    let temp_pfa = unsafe {
        SingleThreadedBumpPFA::new(SingleThreadedLock::new(
            BumpPFA::new(second_alloc_start, second_alloc_end, page_size).unwrap(),
        ))
    };
    println!(
        "Reserved range {:X} - {:X} for bootloader frame allocation",
        kernel_end, second_alloc_end
    );

    // Temporarily identity map bottom 8GiB of address space
    // Regardless of how much RAM the raspi actually has (max 8GiB), this will ensure our identity map
    // acts pretty much the same as before the MMU was enabled.
    println!("Temporarily identity mapping first 4 GiB of address space");
    // SAFETY: This page table will only be used to set up the higher half page tables, so our
    // translation function is always guarunteed to be correct.
    let mut temp_page_table = unsafe { PageTable::new(|phys| phys, &temp_pfa).unwrap() };
    for addr in (0..0x200000000).step_by(1024 * 1024 * 1024) {
        temp_page_table.map_1gib_page(addr, addr, MemoryAttributes::DeviceStronglyOrdered);
    }
    // Construct a higher half page table for ttbr1
    let mut ttbr1 = unsafe { PageTable::new(|phys| phys, &pfa).unwrap() };
    // Map the kernel to the canonical higher half location
    let kernel_phys_start = read_linker_var!(__KERNEL_PHYS_START);
    let kernel_phys_end = read_linker_var!(__KERNEL_PHYS_END);
    if kernel_phys_start % page_size != 0 || kernel_phys_end % page_size != 0 {
        panic!("Kernel start and end must be divisible by page size");
    } else if kernel_phys_start == kernel_phys_end {
        panic!("Kernel section is missing");
    }
    // TODO: Decide on page granularity
    let kernel_virt_start = read_linker_var!(__KERNEL_VIRT_START);
    let mut kernel_virt_page = kernel_virt_start;
    for phys_addr in (kernel_phys_start..kernel_phys_end).step_by(page_size) {
        ttbr1.map_4kib_page(
            kernel_virt_page as u64,
            phys_addr as u64,
            MemoryAttributes::NormalCacheable,
        );

        kernel_virt_page += page_size;
    }
    println!(
        "Mapped kernel to range {:#X} - {:#X}",
        kernel_virt_start, kernel_virt_page
    );
    // Next, map the stack to the higher half
    let stack_phys_end = read_linker_var!(__STACK_END);
    let stack_phys_start = read_linker_var!(__STACK_START);
    let stack_virt_start = kernel_virt_page;
    if stack_phys_end % page_size != 0 || stack_phys_start % page_size != 0 {
        panic!("Stack start and end must be divisible by page size");
    } else if stack_phys_end == stack_phys_start {
        panic!("Stack section is missing (How?!)");
    }
    for phys_addr in (stack_phys_end..stack_phys_start).step_by(page_size) {
        ttbr1.map_4kib_page(
            kernel_virt_page as u64,
            phys_addr as u64,
            MemoryAttributes::NormalCacheable,
        );

        kernel_virt_page += page_size;
    }
    println!(
        "Mapped stack to range {:#X} - {:#X}",
        stack_virt_start, kernel_virt_page
    );
    // Construct a bump allocator with a single page of memory, and map it
    // This will store things like our Arch object and our MemoryMap entries.
    // TODO: Adjust this so it works with non 4kib pages
    let phys_page = (&pfa).allocate_zeroed_pages(1, |x| x).unwrap();
    let mut bump_allocator = unsafe { StaticBumpAlloc::new(phys_page, 4096) };
    ttbr1.map_4kib_page(
        kernel_virt_page as u64,
        phys_page as u64,
        MemoryAttributes::NormalCacheable,
    );
    kernel_virt_page = kernel_virt_page.next_multiple_of(1024 * 1024 * 1024);
    let linear_map_start = kernel_virt_page;
    // Finally, map all of physical memory with 1GiB huge pages, so we have an easy phys -> virt translation
    for addr in (0..0x200000000).step_by(1024 * 1024 * 1024) {
        ttbr1.map_1gib_page(
            kernel_virt_page as u64,
            addr,
            MemoryAttributes::DeviceStronglyOrdered,
        );

        kernel_virt_page += 1024 * 1024 * 1024;
    }
    println!(
        "Linearly mapped physical memory to range {:#X} - {:#X}",
        linear_map_start, kernel_virt_page
    );

    // Prepare the memory map
    let mut mem_map = MemoryMap::<32>::new_in(&mut bump_allocator).unwrap();
    // Query physical memory ranges from dtb
    let dtb = RaspiDeviceTree::new(dtb_ptr).unwrap();
    dtb.for_each_memory(|start, size| {
        mem_map.add_entry(MemoryMapEntry::new(
            start as usize,
            (start + size) as usize,
            MemoryMapType::FREE,
        ));
    });
    // Now start filling up the map with non-free regions
    // Firstly, the first page is reserved because secondary CPUs are parked there
    mem_map.add_entry(MemoryMapEntry::new(0, page_size, MemoryMapType::RESERVED));
    // Next up, the stack
    mem_map.add_entry(MemoryMapEntry::new(
        stack_phys_end,
        stack_phys_start,
        MemoryMapType::STACK,
    ));
    // Next, the kernel
    mem_map.add_entry(MemoryMapEntry::new(
        kernel_phys_start,
        kernel_phys_end,
        MemoryMapType::KERNEL,
    ));
    // Now we have to mark reserved memory for the page tables, etc that will be shared with the kernel
    mem_map.add_entry(MemoryMapEntry::new(
        pfa.allocated_range().0,
        pfa.allocated_range().1,
        MemoryMapType::RESERVED,
    ));
    println!(
        "Printing physical memory map:\n\n\
        Page size:      {}\n\
        Free Memory:    {}\n\
        Free Pages:     {}\n\
        Reserved Pages: {}\n\
        {}",
        MemorySize::new(page_size),
        MemorySize::new(mem_map.get_free_mem()),
        mem_map.get_free_mem() / page_size,
        (mem_map.get_total_mem() - mem_map.get_free_mem()) / page_size,
        mem_map
    );

    print!("Enabling MMU with identity mapping...");
    unsafe {
        enable_mmu(&mut temp_page_table, &mut ttbr1);
    }
    println!("Success");

    println!("Transferring control to kernel...\n");
    let kernel_entry: extern "C" fn() -> ! =
        unsafe { core::mem::transmute(read_linker_var!(__KERNEL_VIRT_START)) };
    kernel_entry();
}

fn early_init_uart() -> Pl011 {
    // Create temp device drivers needed to init the uart
    let mut gpio: Gpio;
    let mut mailbox: Mailbox;
    let uart: Pl011;
    unsafe {
        // Safety: The MMIO addresses are correct for the given Raspberry Pi board revision.
        gpio = Gpio::new(GPIO_PHYS_BASE);
        mailbox = Mailbox::new(MAILBOX_PHYS_BASE);
    }
    // Set the UART frequency to a known value & construct uart driver
    let mut uart_rate_msg = SetClockRate::new(CLOCK_UART, 30000000);
    mailbox.send_property_mail(&mut uart_rate_msg).unwrap();
    unsafe {
        // Safety: The MMIO address is correct and we have set the correct UART clock frequency
        uart = Pl011::new(PL011_PHYS_BASE, &mut gpio);
    }

    uart
}
