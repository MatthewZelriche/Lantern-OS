#[derive(Debug)]
pub enum DeviceError {
    BadWrite,
    Busy,
    BadOperand,
}

#[derive(Debug)]
pub struct AllocError;
