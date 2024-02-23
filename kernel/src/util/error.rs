#[derive(Debug)]
pub enum DeviceError {
    BadWrite,
    Busy,
    BadOperand,
}
