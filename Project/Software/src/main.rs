// main.rs
// Author: PASC
// Version: 0.1.0
// Description:
// Date: 05/03-18
#![no_main]
#![no_std]

extern crate panic_halt;
extern crate stm32f4xx_hal as hal;
use cortex_m::{iprintln};

use rtfm::app;

#[app(device = hal::stm32)]
const APP: () = {
    #[init]
    fn init() {
        let stim = &mut core.ITM.stim[0];
        iprintln!(stim, "Det fungerar!");
    }

    #[idle(resources = [])]
    fn idle() -> ! {
        loop {}
    }
};