use kernel::print::RawWriterMutex;

pub struct SingleThreadedRawWriterMutex;

impl SingleThreadedRawWriterMutex {
    /// Constructs a new RawWriterMutex for use exclusively in Single-Threaded environments
    ///
    /// # Safety
    /// It is unsafe to use this Mutex anywhere except a strictly Single-Threaded environment.
    pub unsafe fn new() -> Self {
        SingleThreadedRawWriterMutex
    }
}

impl RawWriterMutex for SingleThreadedRawWriterMutex {
    fn lock(&self) {}

    fn try_lock(&self) -> bool {
        true
    }

    unsafe fn unlock(&self) {}
}
