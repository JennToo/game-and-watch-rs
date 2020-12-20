#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32h7::stm32h7b3;

#[entry]
fn main() -> ! {
    let peripherals = stm32h7b3::Peripherals::take().unwrap();

    // TODO: This is just junk to make sure `peripherals` doesn't get optimized out yet
    let gpioa = &peripherals.GPIOA;
    gpioa.odr.modify(|_, w| w.odr0().set_bit());

    loop {
        // TODO: Why is the hprintln causing linking errors???
        //hprintln!("Hello from Rust!");
        panic!("Hello from Rust!");
    }
}
