use super::memory_size::MemorySize;
use crate::{
    allocators::{static_box::StaticBox, StaticAlloc},
    util::error::AllocError,
};
use arrayvec::ArrayVec;
use core::{fmt::Display, ops::Deref};

#[derive(Default, Clone, Copy, PartialEq)]
pub enum MemoryMapType {
    #[default]
    FREE,
    RESERVED,
    RECLAIM,
    KERNEL,
    STACK,
    MMIO,
}

#[derive(Default, Clone, Copy)]
pub struct MemoryMapEntry {
    pub base_addr: usize,
    pub end_addr: usize,
    pub mem_type: MemoryMapType,
}

impl MemoryMapEntry {
    pub fn new(base_addr: usize, end_addr: usize, mem_type: MemoryMapType) -> Self {
        Self {
            base_addr,
            end_addr,
            mem_type,
        }
    }

    fn size(&self) -> MemorySize {
        MemorySize::new(self.end_addr - self.base_addr)
    }

    fn contains(&self, other: &Self) -> bool {
        let range = self.base_addr..self.end_addr;
        range.contains(&other.base_addr) || range.contains(&other.end_addr)
    }
    fn fully_contains(&self, other: &Self) -> bool {
        let range = self.base_addr..=self.end_addr;
        range.contains(&other.base_addr) && range.contains(&other.end_addr)
    }

    fn reduce(&mut self, other: &Self) -> Option<MemoryMapEntry> {
        let mut new_block = None;
        if self.contains(other) {
            if other.base_addr <= self.base_addr {
                self.base_addr = other.end_addr;
            } else if other.end_addr >= self.end_addr {
                self.end_addr = other.base_addr;
            } else {
                let old_end = self.end_addr;
                // Truncate original
                self.end_addr = other.base_addr;

                // Add new free after reserved
                let end = old_end;
                new_block = Some(MemoryMapEntry {
                    base_addr: other.end_addr,
                    end_addr: end,
                    mem_type: MemoryMapType::FREE,
                });
            }
        }

        new_block
    }
}

pub struct MemoryMap<const CAP: usize> {
    entries: StaticBox<ArrayVec<MemoryMapEntry, CAP>>,
}

impl<const CAP: usize> MemoryMap<CAP> {
    pub fn new_in<A: StaticAlloc>(allocator: &mut A) -> Result<Self, AllocError> {
        let boxed = StaticBox::new(ArrayVec::new(), allocator)?;
        Ok(Self { entries: boxed })
    }

    pub fn get_free_mem(&self) -> usize {
        let mut bytes = 0;
        for entry in self.entries.deref() {
            match entry.mem_type {
                MemoryMapType::FREE | MemoryMapType::RECLAIM => bytes += entry.size().as_bytes(),
                _ => (),
            }
        }

        bytes
    }

    pub fn get_total_mem(&self) -> usize {
        let mut bytes = 0;
        for entry in self.entries.deref() {
            bytes += entry.size().as_bytes();
        }

        bytes
    }

    pub fn get_entries(&self) -> &ArrayVec<MemoryMapEntry, CAP> {
        &self.entries
    }

    pub fn add_entry(&mut self, mut entry: MemoryMapEntry) -> bool {
        // Merge adjacent entries of same type
        if let Some(old_entry) = self
            .entries
            .iter()
            .find(|x| x.end_addr == entry.base_addr && x.mem_type == entry.mem_type)
        {
            entry.base_addr = old_entry.base_addr;
        }
        if let Some(old_entry) = self
            .entries
            .iter()
            .find(|x| x.base_addr == entry.end_addr && x.mem_type == entry.mem_type)
        {
            entry.end_addr = old_entry.end_addr;
        }

        // Remove free entries if they are completely consumed by a reserved entry
        self.entries.retain(|x| !entry.fully_contains(x));

        // Reduce our free space
        let mut new_entries: ArrayVec<MemoryMapEntry, 4> = ArrayVec::new();
        for existing in self.entries.as_mut_slice() {
            if let Some(additional_entry) = existing.reduce(&entry) {
                new_entries.push(additional_entry);
            }
        }

        // Add any newly created entries to accomodate the reserved entry
        if self.entries.try_extend_from_slice(&new_entries).is_err() {
            return false;
        }

        // Add the reserved entry and return
        if self.entries.try_push(entry).is_err() {
            return false;
        }

        // Sort the map from 0 to max addr
        self.entries
            .sort_unstable_by(|a, b| a.base_addr.cmp(&b.base_addr));

        true
    }
}

impl MemoryMapType {
    fn to_string(&self) -> &str {
        match self {
            MemoryMapType::FREE => "Free",
            MemoryMapType::RESERVED => "Reserved",
            MemoryMapType::RECLAIM => "Reclaim",
            MemoryMapType::KERNEL => "Kernel",
            MemoryMapType::STACK => "Stack",
            MemoryMapType::MMIO => "MMIO",
        }
    }
}

impl Display for MemoryMapEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Type: {:10} | {:#018x} - {:#018x} | {}\n",
            self.mem_type.to_string(),
            self.base_addr,
            self.end_addr,
            self.size()
        )?;

        Ok(())
    }
}

impl<const CAP: usize> Display for MemoryMap<CAP> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for entry in self.entries.deref() {
            if entry.size().as_bytes() != 0 {
                write!(f, "{}", entry)?;
            }
        }
        Ok(())
    }
}
