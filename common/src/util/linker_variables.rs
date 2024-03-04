extern "C" {
    pub static __PG_SIZE: u8;
}

#[macro_export]
macro_rules! read_linker_var {
    ($a:expr) => {
        unsafe { (&$a as *const u8) as usize }
    };
}
