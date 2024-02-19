use crate::util::error::DeviceError;

pub trait CharacterDevice {
    fn write(&mut self, data: &[u8]) -> Result<usize, DeviceError>;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, DeviceError>;
}
