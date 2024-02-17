use super::gpio::Gpio;

pub struct Uart0 {

}

impl Uart0 {
   pub fn new(gpio: &mut Gpio) -> Self {
      // Pins 14 and 15 should be in neither UP now DOWN pull state when using UART0
      gpio.configure_uart0_pull();

      Self {}
   }
}