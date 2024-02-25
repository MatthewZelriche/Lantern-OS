use kernel::print::RawWriterMutex;

pub struct SingleThreadedRawWriterMutex;

impl SingleThreadedRawWriterMutex {
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
