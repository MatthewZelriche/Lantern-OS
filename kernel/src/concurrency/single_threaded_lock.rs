use lock_api::{GuardSend, RawMutex};

pub struct RawSingleThreadedLock;

// Safety: We are guarunteeing to the compiler that this kind of mutex will only ever be used
// in a fully single-threaded environment, including with interrupts disabled
unsafe impl RawMutex for RawSingleThreadedLock {
    const INIT: RawSingleThreadedLock = RawSingleThreadedLock;

    type GuardMarker = GuardSend;

    fn lock(&self) {}

    fn try_lock(&self) -> bool {
        true
    }

    unsafe fn unlock(&self) {}
}

pub type SingleThreadedLock<T> = lock_api::Mutex<RawSingleThreadedLock, T>;
pub type SingleThreadedLockGuard<'a, T> = lock_api::MutexGuard<'a, RawSingleThreadedLock, T>;
