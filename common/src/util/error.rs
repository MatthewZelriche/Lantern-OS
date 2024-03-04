#[derive(Debug)]
pub enum DeviceError {
    BadWrite,
    Busy,
    BadOperand,
    Other,
}

#[derive(Debug)]
pub struct AllocError;
