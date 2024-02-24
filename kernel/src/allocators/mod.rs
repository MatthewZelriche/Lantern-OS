use crate::util::error::AllocError;
use core::{alloc::Layout, ptr::NonNull};

pub mod static_box;
pub mod static_bump;

/// Allocates memory that will live for the rest of the program's lifetime.
///
/// Memory allocated via a StaticAlloc implementation can never be freed. Once allocated, the memory
/// remains allocated for the rest of the program's lifetime.
pub unsafe trait StaticAlloc {
    /// Allocates a block of raw memory satisfying the requirements of layout
    fn allocate_bytes(&mut self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>;
}
