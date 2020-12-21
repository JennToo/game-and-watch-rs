#![no_main]
#![no_std]

use game_and_watch::game_and_watch;

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    game_and_watch(|| {
        hprintln!("Hello from Rust!").unwrap();
    })
}
