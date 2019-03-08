// main.rs
// Author: PASC
// Version: 0.1.0
// Description:
// Date: 05/03-18
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::{iprintln};

extern crate stm32f4xx_hal as hal;
use crate::hal::prelude::*;
use hal::stm32::{ITM};
use hal::gpio::{Analog, gpioa::PA1};

use rtfm::{app};

mod adc;

#[app(device = hal::stm32)]
const APP: () = {
    static mut PA1: PA1<Analog> = ();
    static mut ITM: ITM = ();

    #[init]
    fn init() {
        let rcc = device.RCC;
        let dma2 = device.DMA2;
        let adc1 = device.ADC1;

        let gpioa = device.GPIOA.split();

        let pa1 = gpioa.pa1.into_analog();

        let adc = adc::Adc(&adc1);

        adc.init(&dma2, &rcc);
        adc.enable_input(adc::AdcChannel::_1, 1);
        adc.enable();

        PA1 = pa1;

        ITM = core.ITM;

    }

    #[idle(resources = [ITM])]
    fn idle() -> ! {
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "Det fungerar!");

        loop {}
    }
};