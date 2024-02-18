use kernel::util::register_ref::RegisterRef;
use tock_registers::{interfaces::{ReadWriteable, Writeable}, register_bitfields, register_structs, registers::ReadWrite};

use self::DR::DATA;

use super::{gpio::Gpio, MMIO_BASE};

pub static PL011_PHYS_BASE: usize = MMIO_BASE + 0x201000;

register_structs!(
   pub UartRegisters {
      (0x00 => dr: ReadWrite<u32, DR::Register>),
      (0x04 => reserved0), 
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
   pub unsafe fn new(start_addr: usize, gpio: &mut Gpio) -> Self {
      let registers: RegisterRef<UartRegisters> = RegisterRef::new(start_addr);

      // Disable the UART
      registers.cr.set(0);

      // Pins 14 and 15 should be in neither UP now DOWN pull state when using UART0
      gpio.configure_uart0_pull();

      // TODO: Need to set baud rate before this works on real hardware

      // Enable FIFO queue and 8-bit words
      registers.lcrh.modify(LCRH::FEN::SET);
      registers.lcrh.modify(LCRH::WLEN::EIGHT);

      // Re-enable the UART
      registers.cr.modify(CR::UARTEN::SET);
      registers.cr.modify(CR::TXE::SET);
      registers.cr.modify(CR::RXE::SET);
      Self {
         registers
      }
   }

   pub fn write_byte(&mut self, byte: u8) {
      self.registers.dr.modify(DATA.val(byte as u32));
   }
}