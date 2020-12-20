#![no_main]
#![no_std]

use game_and_watch::GameAndWatch;

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let mut system = GameAndWatch::init();

    loop {
        hprintln!("Hello from Rust!").unwrap();
        system.update();
    }
}
