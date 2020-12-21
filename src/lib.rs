#![no_std]

use stm32h7::stm32h7b3::Peripherals;
use stm32h7xx_hal::prelude::*;

pub fn game_and_watch<F>(user_code: F) -> !
where
    F: Fn(),
{
    let hardware = Peripherals::take().unwrap();

    // TODO: vos0 support would be nice
    let pwr = hardware.PWR.constrain();
    let pwrconfig = pwr.ldo().freeze();

    let rcc = hardware.RCC.constrain();
    let ccdr = rcc
        .sys_ck(280.mhz())
        .pll3_p_ck(72.mhz())
        .pll3_q_ck(72.mhz())
        .pll3_r_ck(6.mhz()) // LTDC Pixel clock
        .freeze(pwrconfig, &hardware.SYSCFG);

    loop {
        user_code();
    }
}
