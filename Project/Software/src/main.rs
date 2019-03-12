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
use hal::stm32::{ITM, DMA2};

use rtfm::{app, Instant};

mod adc;
mod pwm;
mod frequency;
mod time;
mod dma;
mod channel;

use time::Hertz;
use channel::Channel;
use dma::{CircBuffer, Dma2Stream0};

const FREQUENCY: Hertz = Hertz(1000);
const ADCFREQUENCY: Hertz = Hertz(1);
const N: usize = 2;
// Our error type
#[derive(Debug)]
pub enum Error {
    RingBufferOverflow,
    UsartSendOverflow,
    UsartReceiveOverflow,
}

#[app(device = hal::stm32)]
const APP: () = {
    static mut ITM: ITM = ();
    static mut DMA2: DMA2 = ();
    // static mut ADC: adc::Adc = ();
    static mut BUFFER: CircBuffer<'static, [u16; N], Dma2Stream0> = CircBuffer::new([[0; N]; 2]);
    #[init(schedule = [read_adc], resources = [BUFFER])]
    fn init() {
        let rcc = device.RCC;
        let dma2 = device.DMA2;
        let adc1 = device.ADC1;
        let tim2 = device.TIM2;


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

        let c2 = &Channel::_2;
        let mut pwm2 = pwm::Pwm(&tim2);
        pwm2.init(
            ADCFREQUENCY.invert(),
            *c2,
            None,
            &device.GPIOA,
            &device.GPIOB,
            &device.GPIOC,
            &rcc,
        );
        
        let adc = adc::Adc(&adc1);

        adc.init(&dma2, &rcc);
        adc.enable_input(adc::AdcChannel::_0, 1, &device.GPIOA, &device.GPIOB, &device.GPIOC);
        adc.enable_input(adc::AdcChannel::_1, 2, &device.GPIOA, &device.GPIOB, &device.GPIOC);
        adc.enable();
        adc.start(resources.BUFFER, &dma2, &mut pwm2).unwrap();
       
        schedule.read_adc(Instant::now() + (12_000_000).cycles()).unwrap();
        
        ITM = core.ITM;
        DMA2 = dma2;
        
        // ADC = adc;
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

    //Todo, implement this to trigger on interrupt when buffer is readable or on adc conversion.
    #[task(resources = [ITM, BUFFER, DMA2], schedule = [read_adc])]
    fn read_adc() {
        let stim = &mut resources.ITM.stim[0];
        match resources.BUFFER.read(resources.DMA2, |x| {
                let buf: [u16; N] = x.clone();
                buf
            }) {
                Err(_) => {
                    iprintln!
                    (stim, "Error reading buffer");
                },
                Ok(b) => {
                    iprintln!(stim, "Temp = {}, Pulse = {}", b[0], b[1]);
                }
            }
        schedule.read_adc(Instant::now() + (12_000_000).cycles()).unwrap();
    }
    // Set of interrupt vectors, free to use for RTFM tasks
    // 1 per priority level suffices

    // #[interrupt]
    // fn ADC() {
    //     let stim = &mut resources.ITM.stim[0];
    //     match resources.BUFFER.read(resources.DMA2, |x| {
    //             let buf: [u16; N] = x.clone();
    //             buf
    //         }) {
    //             Err(_) => cortex_m::asm::bkpt(),
    //             Ok(b) => {
    //                 iprintln!(stim, "{}, {}, {}, {}, {}, {}", b[0], b[1], b[2], b[3], b[4], b[5]);
    //             }
    //         }
    // }
    extern "C" {
        fn EXTI0();
    }
};