use kernel::util::{error::DeviceError, register_ref::RegisterRef};
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{InMemoryRegister, ReadWrite},
};

use self::message::{MailboxMessageData, Message, STATUS_FAILURE};

use super::MMIO_BASE;

pub mod message;

pub const MAILBOX_PHYS_BASE: usize = MMIO_BASE + 0xB880;
pub const MAILBOX_0_PHYS_BASE: usize = MAILBOX_PHYS_BASE + 0x0;
pub const MAILBOX_1_PHYS_BASE: usize = MAILBOX_PHYS_BASE + 0x20;

register_structs!(
   pub MailboxRegisters {
      (0x00 => data: ReadWrite<u32, DATA::Register>),
      (0x04 => reserved0),
      (0x18 => status: ReadWrite<u32, STATUS::Register>),
      (0x1C => reserved1),
      (0x24 => @END),
   }
);

register_bitfields!(
   u32,

   DATA [
      CHANNEL OFFSET(0) NUMBITS(4),
      ADDR OFFSET(4) NUMBITS(28),
   ],

   STATUS [
      LEVEL OFFSET(0) NUMBITS(8),
      EMPTY OFFSET(30) NUMBITS(1),
      FULL OFFSET(31) NUMBITS(1),
   ]
);

pub struct Mailbox {
    mbox_0: RegisterRef<MailboxRegisters>,
    mbox_1: RegisterRef<MailboxRegisters>,
}

impl Mailbox {
    pub unsafe fn new(start_addr: usize) -> Self {
        Self {
            mbox_0: RegisterRef::new(start_addr),
            mbox_1: RegisterRef::new(start_addr + 0x20),
        }
    }

    /// Sends a single message to the VideoCore mailbox, blocking until a reply is received
    ///
    /// Note that the mailbox can only fit 32 bit addresses into its register, so caller must verify
    /// that the address of their message is below 0x100000000. The mailbox also requires physical addresses,
    /// so if virtual memory mapping is enabled a lookup from virt -> phys will be necessary first
    pub fn send_property_mail<T: MailboxMessageData>(
        &mut self,
        message: &mut Message<T>,
    ) -> Result<(), DeviceError> {
        let message_ptr: *mut Message<T> = message;
        // If the CPU receiving queue is full, we can't send a message or we risk losing the reply
        // if the queue hasn't freed up space by the time the GPU services the request.
        if self.mbox_0.status.is_set(STATUS::FULL) {
            return Err(DeviceError::Busy);
        }
        // message_addr MUST be a physical address below 4GiB! It cannot be greater than 32 bits.
        if message_ptr as usize >= 0x100000000 {
            return Err(DeviceError::BadOperand);
        }

        // Write mailbox address upper 28 bits into mbox_1 read register
        // Write in channel 8, the property channel, the only channel supported
        let data: InMemoryRegister<u32, DATA::Register> = InMemoryRegister::new(0);
        data.modify(DATA::ADDR.val(message_ptr as u32 >> 4));
        data.modify(DATA::CHANNEL.val(8));

        // Send data to the VideoCore
        self.mbox_1.data.set(data.get());

        // Block until a message is received from the VideoCore on channel 8
        while self.mbox_0.status.is_set(STATUS::EMPTY) || self.mbox_0.data.read(DATA::CHANNEL) != 8
        {
        }

        if message.status == STATUS_FAILURE {
            Err(DeviceError::Other)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        message::{GetClockRate, SetClockRate, STATUS_FAILURE, STATUS_SUCCESS},
        Mailbox, MAILBOX_PHYS_BASE,
    };
    use kernel::{kprint, kprintln};

    #[test_case]
    fn mailbox_tests() {
        kprint!("Testing VideoCore Mailbox device driver...");

        let mut mailbox = unsafe { Mailbox::new(MAILBOX_PHYS_BASE) };

        // Try setting the clock rate for the UART...
        let new_clock_rate = 3000000;
        let mut message = SetClockRate::new(2, new_clock_rate);
        mailbox.send_property_mail(&mut message).unwrap();

        // Read the new clock rate to confirm it is correct
        let mut message = GetClockRate::new(2);
        mailbox.send_property_mail(&mut message).unwrap();
        assert_eq!(message.data.rate, new_clock_rate);

        kprintln!("Success!");
    }
}
