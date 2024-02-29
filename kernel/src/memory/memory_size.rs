/// Convenience struct for converting between different memory units
pub struct MemorySize {
    bytes: usize,
}

pub const KIB_SIZE: usize = 1024;
pub const MIB_SIZE: usize = KIB_SIZE * 1024;
pub const GIB_SIZE: usize = MIB_SIZE * 1024;
pub const TIB_SIZE: usize = GIB_SIZE * 1024;

impl MemorySize {
    /// Constructs a new MemorySize from a given number of bytes
    pub fn new(bytes: usize) -> Self {
        Self { bytes }
    }

    pub fn from_kibibytes(kibibytes: usize) -> Self {
        Self {
            bytes: kibibytes * KIB_SIZE,
        }
    }

    pub fn from_mebibytes(mebibytes: usize) -> Self {
        Self {
            bytes: mebibytes * MIB_SIZE,
        }
    }

    pub fn from_gibibytes(gibibytes: usize) -> Self {
        Self {
            bytes: gibibytes * GIB_SIZE,
        }
    }

    pub fn from_tebibytes(tebibytes: usize) -> Self {
        Self {
            bytes: tebibytes * GIB_SIZE,
        }
    }

    pub fn as_bytes(&self) -> usize {
        self.bytes
    }

    pub fn as_kibibytes(&self) -> f64 {
        self.bytes as f64 / KIB_SIZE as f64
    }

    pub fn as_mebibytes(&self) -> f64 {
        self.bytes as f64 / MIB_SIZE as f64
    }

    pub fn as_gibibytes(&self) -> f64 {
        self.bytes as f64 / GIB_SIZE as f64
    }

    pub fn as_tebibytes(&self) -> f64 {
        self.bytes as f64 / TIB_SIZE as f64
    }
}
