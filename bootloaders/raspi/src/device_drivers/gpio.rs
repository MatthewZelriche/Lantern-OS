use core::arch::asm;

use kernel::util::register_ref::RegisterRef;
use tock_registers::{
    interfaces::ReadWriteable, register_bitfields, register_structs, registers::ReadWrite,
};

use self::{
    GPIO_PUP_PDN_CNTRL_REG0::{GPIO_PUP_PDN_CNTRL14, GPIO_PUP_PDN_CNTRL15},
    GPPUD::PUD,
    GPPUDCLK0::{PUDCLK14, PUDCLK15},
};

use super::MMIO_BASE;

pub const GPIO_PHYS_BASE: usize = MMIO_BASE + 0x200000;

register_structs! {
   pub GpioRegisters {
      (0x00 => reserved0),
      (0x94 => gppud: ReadWrite<u32, GPPUD::Register>),  // RPI3 only
      (0x98 => gppudclk0: ReadWrite<u32, GPPUDCLK0::Register>), // RPI3 only,
      (0x9C => reserved1),
      (0xE4 => pup_pdn_ctrl_reg0: ReadWrite<u32, GPIO_PUP_PDN_CNTRL_REG0::Register>),  // RPI4 only
      (0xE8 => reserved2),
      (0xF4 => @END),
   }
}

register_bitfields!(
   u32,

   GPPUD [
      PUD OFFSET(0) NUMBITS(2) [
         NONE = 0,
         DOWN = 1,
         UP = 2,
         RESERVED = 3,
      ]
   ],

   GPPUDCLK0 [
      PUDCLK14 OFFSET(14),
      PUDCLK15 OFFSET(15),
   ],

   GPIO_PUP_PDN_CNTRL_REG0 [
      GPIO_PUP_PDN_CNTRL14 OFFSET(28) NUMBITS (2) [
         NONE = 0,
         UP = 1,
         DOWN = 2,
         RESERVED = 3,
      ],
      GPIO_PUP_PDN_CNTRL15 OFFSET(30) NUMBITS (2) [
         NONE = 0,
         UP = 1,
         DOWN = 2,
         RESERVED = 3,
      ],
   ],
);

pub struct Gpio {
    registers: RegisterRef<GpioRegisters>,
}

impl Gpio {
    /// Creates a new representation of the General Purpose I/O pins
    ///
    /// This acts essentially as a simple wrapper around a chunk of MMIO specified by start_addr,
    /// so that certain useful register manipulations can be abstracted away into method calls.
    ///
    /// # Safety
    /// start_addr must be dereferencable to GpioRegisters (ie, it must point to the correct start address in MMIO).
    pub unsafe fn new(start_addr: usize) -> Self {
        Self {
            registers: RegisterRef::new(start_addr),
        }
    }

    #[cfg(feature = "raspi3")]
    /// Configures the GPIO pins 14 and 15 to be neither UP nor DOWN, as expected by UART0
    pub fn configure_uart0_pull(&mut self) {
        // We have to set pins 14 and 15 to neither pull up nor pull down
        // Yet I still don't quite understand why this specific sequence of register writes
        // It seems to be something to do with using GPPUD to specify the mode you want, then using
        // GPPUDCLKn as a mask for which pins to modify?

        // Note also that we need to spin for 150 clock cycles according to the documentation
        // Ideally, we would use the microsecond clock, but to avoid pulling in additional things that can
        // go wrong during the initialization of something as important as UART0, I elected to just spin
        // nops inside a for loop. This definitely overshoots 150 cycles but its only done once or twice
        // during initialization so it should be imperceptible.

        self.registers.gppud.modify(PUD::NONE);
        unsafe {
            for i in [0..150] {
                asm!("nop");
            }
        }
        self.registers.gppudclk0.modify(PUDCLK14::SET);
        self.registers.gppudclk0.modify(PUDCLK15::SET);
        unsafe {
            for i in [0..150] {
                asm!("nop");
            }
        }
        // Docs tell us to clear gppud here, but its already set to 0 so surely it doesn't matter?
        self.registers.gppudclk0.modify(PUDCLK14::CLEAR);
        self.registers.gppudclk0.modify(PUDCLK15::CLEAR);
    }

    #[cfg(feature = "raspi4")]
    /// Configures the GPIO pins 14 and 15 to be neither UP nor DOWN, as expected by UART0
    pub fn configure_uart0_pull(&mut self) {
        self.registers
            .pup_pdn_ctrl_reg0
            .modify(GPIO_PUP_PDN_CNTRL14::NONE);
        self.registers
            .pup_pdn_ctrl_reg0
            .modify(GPIO_PUP_PDN_CNTRL15::NONE);
    }
}
