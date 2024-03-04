/// Essentially a carbon copy of lock_api's RawMutex, but without the const associated variable. We needed
/// to strip that so that we can make it object safe for dyn.
pub trait RawWriterMutex: Send {
    fn lock(&self);
    fn try_lock(&self) -> bool;
    unsafe fn unlock(&self);
}
