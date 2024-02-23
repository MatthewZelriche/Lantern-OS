use core::mem::size_of;

#[repr(C, align(16))]
pub struct Message<T> {
    msgsize: u32,
    pub status: u32,
    pub data: T,
    null: u32,
}

pub trait MailboxMessageData {}

#[derive(Default)]
#[repr(C)]
pub struct GetArmMemory {
    tag: u32,
    bufsize: u32,
    status: u32,
    pub base: u32,
    pub size: u32,
}

#[derive(Default)]
#[repr(C)]
pub struct GetClockRate {
    tag: u32,
    bufsize: u32,
    status: u32,
    pub base: u32,
    pub size: u32,
}

impl MailboxMessageData for GetArmMemory {}
impl MailboxMessageData for GetClockRate {}

impl GetArmMemory {
    pub fn new() -> Message<Self> {
        Message {
            msgsize: size_of::<Message<Self>>().try_into().unwrap(),
            status: 0,
            data: GetArmMemory {
                tag: 0x00010005,
                bufsize: 8,
                status: 0,
                base: 0,
                size: 0,
            },
            null: 0,
        }
    }
}
