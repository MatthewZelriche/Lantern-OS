use fdt_rs::{
    base::DevTree,
    error::DevTreeError,
    prelude::{FallibleIterator, PropReader},
};
use kernel::kprint;

pub struct RaspiDeviceTree<'a> {
    address_cells: u32,
    size_cells: u32,
    dt: DevTree<'a>,
}

impl RaspiDeviceTree<'_> {
    pub fn new(dtb_ptr: *const u8) -> Result<Self, DevTreeError> {
        // Safety: The library will verify that the correct magic number is present. This memory is part
        // of the first physical page which the OS will treat as read only, so the memory is essentially
        // of a static lifetime.
        let dt = unsafe { DevTree::from_raw_pointer(dtb_ptr)? };

        // Address and size values are stored as a series of u32 values, we need to know their length for
        // this devicetree.
        let root = dt.root()?.ok_or(DevTreeError::ParseError)?;
        let address_cells = root
            .props()
            .find(|x| Ok(x.name()? == "#address-cells"))?
            .ok_or(DevTreeError::ParseError)?
            .u32(0)?;
        let size_cells = root
            .props()
            .find(|x| Ok(x.name()? == "#size-cells"))?
            .ok_or(DevTreeError::ParseError)?
            .u32(0)?;

        Ok(Self {
            address_cells,
            size_cells,
            dt,
        })
    }

    /// Iterates over the device tree, parsing each memory region and passing the base address and size in bytes
    /// to the provided closure.
    pub fn for_each_memory<F: Fn(u64, u64)>(&self, closure: F) {
        let mut node_iter = self.dt.nodes();
        // Find the next memory node in the tree
        while let Ok(Some(node)) = node_iter.find(|x| Ok(x.name()?.starts_with("memory@"))) {
            // Recover its properties
            if let Ok(Some(regs)) = node.props().find(|x| Ok(x.name()? == "reg")) {
                let entry_size_cells = self.address_cells + self.size_cells;
                let entry_size_bytes = (entry_size_cells * 4) as usize;
                // There may be more than one address/size memory region in a single node
                for node_entry in 0..regs.raw().len() / entry_size_bytes {
                    // This monstrosity is the best way I could think to parse each address/size, considering
                    // we don't know the cell size of these properties until runtime...
                    let address = match self.address_cells {
                        1 => regs
                            .u32(node_entry * entry_size_cells as usize)
                            .ok()
                            .map(|x| x as u64),
                        2 => regs.u64(node_entry * entry_size_cells as usize).ok(),
                        _ => None,
                    };
                    let size = match self.size_cells {
                        1 => regs
                            .u32(
                                (node_entry * entry_size_cells as usize)
                                    + self.address_cells as usize,
                            )
                            .ok()
                            .map(|x| x as u64),
                        2 => regs
                            .u64(
                                (node_entry * (entry_size_cells / 2) as usize)
                                    + self.address_cells as usize,
                            )
                            .ok(),
                        _ => None,
                    };

                    // Pass the data to the caller if we were able to successfully parse the memory region
                    if address.is_some() && size.is_some() {
                        closure(address.unwrap(), size.unwrap());
                    }
                }
            }
        }
    }
}
