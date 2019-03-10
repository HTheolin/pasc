// main.rs
// Author: PASC
// Version: 0.1.0
// Description:
// Date: 05/03-18
#![no_main]
#![no_std]
#![feature(unsize)]

extern crate panic_halt;

use cortex_m::{asm, iprintln};

extern crate stm32f4xx_hal as hal;
use crate::hal::prelude::*;
use hal::stm32::{ITM};
use hal::gpio::{Analog, gpioa::PA1};

use rtfm::{app, Instant};

mod adc;
mod pwm;
mod frequency;
mod time;
mod dma;

use pwm::Channel;
use time::Hertz;

const FREQUENCY: Hertz = Hertz(1000);

// Our error type
#[derive(Debug)]
pub enum Error {
    RingBufferOverflow,
    UsartSendOverflow,
    UsartReceiveOverflow,
}

#[app(device = hal::stm32)]
const APP: () = {
    static mut PA1: PA1<Analog> = ();
    static mut ITM: ITM = ();
    
    #[init(schedule = [trace])]
    fn init() {
        let rcc = device.RCC;
        let dma2 = device.DMA2;
        let adc1 = device.ADC1;
        let tim2 = device.TIM2;

        let adc = adc::Adc(&adc1);

        let mut pwm = pwm::Pwm(&tim2);

        let c = &Channel::_1;
        pwm.init(
            FREQUENCY.invert(),
            *c,
            None,
            &device.GPIOA,
            &device.GPIOB,
            &device.GPIOC,
            &rcc,
        );
        pwm.set_duty(*c, pwm.get_max_duty() / 16);
        pwm.enable(*c);

        adc.init(&dma2, &rcc);
        adc.enable_input(adc::AdcChannel::_1, 1);
        adc.enable();

       
        let gpioa = device.GPIOA.split();
        let pa1 = gpioa.pa1.into_analog();

        PA1 = pa1;
        
        ITM = core.ITM;

    }

    #[idle()]
    fn idle() -> ! {
        loop {
            asm::wfi();
        }
    }

    #[task(resources = [ITM], schedule = [trace])]
    fn trace() {
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "Det fungerar!");


        schedule.trace(Instant::now() + (16_000_000).cycles()).unwrap();
    }


    // Set of interrupt vectors, free to use for RTFM tasks
    // 1 per priority level suffices
    extern "C" {
        fn EXTI0();
    }
};