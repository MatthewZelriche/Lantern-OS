use core::fmt::Write;

use kernel::{device_drivers::character_device::CharacterDevice, util::register_ref::RegisterRef};
use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use self::DR::DATA;

use super::{gpio::Gpio, MMIO_BASE};

pub static PL011_PHYS_BASE: usize = MMIO_BASE + 0x201000;

register_structs!(
   pub UartRegisters {
      (0x00 => dr: ReadWrite<u32, DR::Register>),
      (0x04 => reserved0),
      (0x18 => fr: ReadOnly<u32, FR::Register>),
      (0x1C => reserved1),
      (0x24 => ibrd: ReadWrite<u32, IBRD::Register>),
      (0x28 => fbrd: ReadWrite<u32, FBRD::Register>),
      (0x2C => lcrh: ReadWrite<u32, LCRH::Register>),
      (0x30 => cr: ReadWrite<u32, CR::Register>),
      (0x34 => @END),
   }
);

register_bitfields!(
   u32,

   DR [
      DATA OFFSET(0) NUMBITS(8),
      FE OFFSET(8) NUMBITS(1),
      PE OFFSET(9) NUMBITS(1),
      BE OFFSET(10) NUMBITS(1),
      OE OFFSET(11) NUMBITS(1),
   ],

   FR [
      CTS OFFSET(0),
      BUSY OFFSET(3),
      RXFE OFFSET(4),
      TXFF OFFSET(5),
      RXFF OFFSET(6),
      TXFE OFFSET(7),
   ],

   IBRD [
      DIV OFFSET(0) NUMBITS(16),
   ],

   FBRD [
    DIV OFFSET(0) NUMBITS(6),
   ],

   LCRH [
      BRK  OFFSET(0) NUMBITS(1) [],
      PEN OFFSET(1) NUMBITS(1) [],
      EPS  OFFSET(2) NUMBITS(1) [],
      STP2 OFFSET(3) NUMBITS(1) [],
      FEN OFFSET(4) NUMBITS(1) [],
      WLEN OFFSET(5) NUMBITS(2) [
          FIVE = 0,
          SIX = 1,
          SEVEN = 2,
          EIGHT = 3
      ]
   ],

   CR [
      UARTEN OFFSET(0),
      LBE OFFSET(7),
      TXE OFFSET(8),
      RXE OFFSET(9),
   ]
);

/// Device driver for the PL011 UART0 on pins 14 & 15 of the raspi3 and raspi4
///
/// Note that this device driver only works for UART0 on the raspi4, and not any of the other UARTs.
pub struct Pl011 {
    registers: RegisterRef<UartRegisters>,
}

impl Pl011 {
    /// Creates a new representation of the Pl011 UART0 device.
    ///
    /// Note: Currently the device driver assumes a UART clock rate of 3 Mhz, so this must be set
    /// prior to calling this method
    pub unsafe fn new(start_addr: usize, gpio: &mut Gpio) -> Self {
        let registers: RegisterRef<UartRegisters> = RegisterRef::new(start_addr);

        // Disable the UART
        registers.cr.set(0);

        // Pins 14 and 15 should be in neither UP now DOWN pull state when using UART0
        gpio.configure_uart0_pull();

        // TODO: Need to set baud rate before this works on real hardware
        // Baud rate calculations from https://wiki.osdev.org/Raspberry_Pi_Bare_Bones
        registers.ibrd.modify(IBRD::DIV.val(1));
        registers.fbrd.modify(FBRD::DIV.val(40));

        // Enable FIFO queue and 8-bit words
        registers.lcrh.modify(LCRH::FEN::SET);
        registers.lcrh.modify(LCRH::WLEN::EIGHT);

        // Re-enable the UART
        registers.cr.modify(CR::UARTEN::SET);
        registers.cr.modify(CR::TXE::SET);
        registers.cr.modify(CR::RXE::SET);
        Self { registers }
    }

    pub fn write_byte(&mut self, byte: u8) {
        // Block until the transmit FIFO has free space...
        while self.registers.fr.is_set(FR::TXFF) {}

        self.registers.dr.modify(DATA.val(byte as u32));
    }

    pub fn would_read_block(&self) -> bool {
        self.registers.fr.is_set(FR::RXFE)
    }

    pub fn read_byte(&self) -> u8 {
        // Block until the receive FIFO has at least one byte...
        while self.registers.fr.is_set(FR::RXFE) {}

        self.registers.dr.read(DR::DATA) as u8
    }
}

impl CharacterDevice for Pl011 {
    fn write(&mut self, data: &[u8]) -> Result<usize, kernel::util::error::DeviceError> {
        for byte in data {
            self.write_byte(*byte);
        }

        Ok(data.len())
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, kernel::util::error::DeviceError> {
        let mut bytes_read = 0;

        for byte in buf {
            // Read at least one byte, even if we have to block
            *byte = self.read_byte();
            bytes_read += 1;

            // Now that we've read at least one byte, bail if there's no more to read
            if self.would_read_block() {
                break;
            }
        }

        Ok(bytes_read)
    }
}

impl Write for Pl011 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes())
            .map_err(|_| core::fmt::Error::default())?;

        Ok(())
    }
}
