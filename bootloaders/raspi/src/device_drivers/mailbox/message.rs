use core::mem::size_of;

pub const STATUS_SUCCESS: u32 = 0x80000000;
pub const STATUS_FAILURE: u32 = 0x80000001;

pub const CLOCK_UART: u32 = 2;

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
    pub id: u32,
    pub rate: u32,
}

#[derive(Default)]
#[repr(C)]
pub struct SetClockRate {
    tag: u32,
    bufsize: u32,
    status: u32,
    pub id: u32,
    pub rate: u32,
    pub skip_turbo: u32,
}

impl MailboxMessageData for GetArmMemory {}
impl MailboxMessageData for GetClockRate {}
impl MailboxMessageData for SetClockRate {}

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

impl GetClockRate {
    pub fn new(id: u32) -> Message<Self> {
        Message {
            msgsize: size_of::<Message<Self>>().try_into().unwrap(),
            status: 0,
            data: Self {
                tag: 0x00030002,
                bufsize: 8,
                status: 0,
                id,
                rate: 0,
            },
            null: 0,
        }
    }
}

impl SetClockRate {
    pub fn new(id: u32, rate: u32) -> Message<Self> {
        Message {
            msgsize: size_of::<Message<Self>>().try_into().unwrap(),
            status: 0,
            data: Self {
                tag: 0x00038002,
                bufsize: 12,
                status: 0,
                id,
                rate,
                skip_turbo: 0,
            },
            null: 0,
        }
    }
}
