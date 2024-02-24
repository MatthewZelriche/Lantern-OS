use crate::device_drivers::character_device::CharacterDevice;

/// A CharacterDevice that is being internally protected by a mutex
///
/// This trait is necessary so that we can assign the static GlobalWriter variable at runtime without knowing
/// at compile time what Mutex implementation we plan to use. This allows us to set GlobalWriter to use
/// a SingleThreadedLock during early init, and a regular SpinLock after early init.
pub trait MutexedWriter: CharacterDevice + Sync {}
