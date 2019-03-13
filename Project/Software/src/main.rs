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
use hal::stm32::{ITM, DMA2, EXTI};
use hal::gpio::gpioc::{PC7, PC8, PC9};
use hal::gpio::gpiob::{PB0, PB1, PB2};
use hal::gpio::{Input, PullDown, ExtiPin};
use rtfm::{app, Instant};

mod adc;
mod pwm;
mod frequency;
mod time;
mod dma;
mod channel;
mod button;

use time::Hertz;
use channel::Channel;
use dma::{CircBuffer, Dma2Stream0};
//use button::{BUTTON, PB0};
const FREQUENCY: Hertz = Hertz(100);
const ADCFREQUENCY: Hertz = Hertz(8);
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
    static mut EXTI: EXTI = ();
    static mut BPC7: PC7<Input<PullDown>>  = ();
    static mut BPC8: PC8<Input<PullDown>>  = ();
    static mut BPC9: PC9<Input<PullDown>>  = ();
    //static mut PC7: PC7<Input<PullDown>> = ();
    // static mut BUTTON: PB0 = ();
    // static mut ADC: adc::Adc = ();
    static mut BUFFER: CircBuffer<'static, [u16; N], Dma2Stream0> = CircBuffer::new([[0; N]; 2]);
    #[init(resources = [BUFFER])]
    fn init() {
        let rcc = device.RCC;
        let dma2 = device.DMA2;
        let adc1 = device.ADC1;
        let tim1 = device.TIM1;
        let tim2 = device.TIM2;
        let mut exti = device.EXTI;
        let mut syscfg = device.SYSCFG;

        // //Enable pwm for driving the piezo speaker, tim2 channel 1 = PA15
        // let mut pwm = pwm::Pwm(&tim2);
        // let c = &Channel::_1;
        // pwm.init(
        //     FREQUENCY.invert(),
        //     *c,
        //     None,
        //     &device.GPIOA,
        //     &device.GPIOB,
        //     &device.GPIOC,
        //     &rcc,
        // );
        // pwm.set_duty(*c, pwm.get_max_duty() / 2);
        // pwm.enable(*c);

        //Enable ADC converting on adc channel IN_0 (PA0) and IN_1 (PA1), uses pwm to trigger converting and uses DMA2_STREAM0
        //interrupt to print values from the buffer.
        //Todo, possibly enable analog watchdog for adc channel 1 (pulse) to generate interrupt on high values to count pulses.
        let c2 = &Channel::_2;
        let mut pwm2 = pwm::Pwm(&tim1);
        pwm2.init(
            ADCFREQUENCY.invert(),
            *c2,
            None,
            &device.GPIOA,
            &device.GPIOB,
            &device.GPIOC,
            &rcc,
        );
        let adc = adc::Adc(&adc1, &tim1);
        adc.init(&dma2, &rcc);
        adc.enable_input(adc::AdcChannel::_0, 1, &device.GPIOA, &device.GPIOB, &device.GPIOC);
        adc.enable_input(adc::AdcChannel::_1, 2, &device.GPIOA, &device.GPIOB, &device.GPIOC);
        adc.enable();
        adc.start(resources.BUFFER, &dma2, &mut pwm2).unwrap();
       
        //button::init(&device.GPIOB, &rcc, &device.SYSCFG, &device.EXTI);
        rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());
        let gpioc = device.GPIOC.split();
        
        
        let button_pc7 = button::Button(gpioc.pc7);
        let button_pc7 = button_pc7.init(&mut syscfg, &mut exti);
        let button_pc8 = button::Button(gpioc.pc8);
        let button_pc8 = button_pc8.init(&mut syscfg, &mut exti);
        let button_pc9 = button::Button(gpioc.pc9);
        let button_pc9 = button_pc9.init(&mut syscfg, &mut exti);
        // let mut pc7  = pc7.into_pull_down_input();

        // pc7.make_interrupt_source(&mut syscfg);
        // pc7.trigger_on_edge(&mut exti, Edge::FALLING);
        // pc7.enable_interrupt(&mut exti);
        ITM = core.ITM;
        DMA2 = dma2;
        EXTI = exti;
        BPC7 = button_pc7;
        BPC8 = button_pc8;
        BPC9 = button_pc9;
        // BUTTON = button::BUTTON;
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

    #[interrupt(resources = [ITM, BUFFER, DMA2])]
    fn DMA2_STREAM0() {
        let stim = &mut resources.ITM.stim[0];
        match resources.BUFFER.read(resources.DMA2, |x| {
                let buf: [u16; N] = x.clone();
                buf
            }) {
                Err(_) => cortex_m::asm::bkpt(),
                Ok(b) => {
                    iprintln!(stim, "{}, {}", b[0], b[1]);
                }
            }
    }

    #[interrupt(resources = [ITM, EXTI])]
    fn EXTI0() {
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "Button was clicked!");
        //BUTTON.clear_pending(&resources.EXTI);
    }
    
    #[interrupt(resources = [ITM, EXTI, BPC7, BPC8, BPC9])]
    fn EXTI9_5() {
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "Button was clicked!");
        resources.BPC7.clear_interrupt_pending_bit(&mut resources.EXTI)
    }


    extern "C" {
        fn EXTI1();
    }
};