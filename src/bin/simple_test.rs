#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32h7::stm32h7b3;

#[entry]
fn main() -> ! {
    let _peripherals = stm32h7b3::Peripherals::take().unwrap();

    loop {
        hprintln!("Hello from Rust!").unwrap();
    }
}
