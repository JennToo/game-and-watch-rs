#![no_std]

use stm32h7::stm32h7b3::Peripherals;

pub struct GameAndWatch {
    /// Raw access to the hardware
    pub hardware: Peripherals,
    // TODO: LCD buffer
    // TODO: Audio buffer
}

impl GameAndWatch {
    /// Initialize hardware and return a system handle.
    ///
    /// You can only call this once! If you call it more than once, this function will panic.
    pub fn init() -> GameAndWatch {
        let hardware = Peripherals::take().unwrap();

        // TODO: Actually initialize stuff

        GameAndWatch { hardware }
    }

    /// Call this once per frame to "do stuff"
    // TODO: Do stuff
    pub fn update(&mut self) {}
}
