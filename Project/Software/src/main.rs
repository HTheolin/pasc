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
use hal::gpio::{Input, PullDown, ExtiPin, Edge};
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
    static mut PC7: PC7<Input<PullDown>>  = ();
    static mut PC8: PC8<Input<PullDown>>  = ();
    static mut PC9: PC9<Input<PullDown>>  = ();

    // static mut PB0: PB0<Input<PullDown>>  = ();
    // static mut PB1: PB1<Input<PullDown>>  = ();
    // static mut PB0: PB2<Input<PullDown>>  = ();
    
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
        let timstart = button::Button(gpioc.pc7).init(&mut syscfg, &mut exti, Edge::FALLING);
        let timinc = button::Button(gpioc.pc8).init(&mut syscfg, &mut exti, Edge::FALLING);
        let timres = button::Button(gpioc.pc9).init(&mut syscfg, &mut exti, Edge::FALLING);
        
        // let gpiob = device.GPIOB.split();
        // let timstart = button::Button(gpiob.pb0).init(&mut syscfg, &mut exti, Edge::FALLING);
        // let timinc = button::Button(gpiob.pb1).init(&mut syscfg, &mut exti, Edge::FALLING);
        // let timres = button::Button(gpiob.pb2).init(&mut syscfg, &mut exti, Edge::FALLING);
 
        PC7 = timstart;
        PC8 = timinc;
        PC9 = timres;
        // PB0 = timstart;
        // BP1 = timinc;
        // BP2 = timres;
        
        ITM = core.ITM;
        DMA2 = dma2;
        EXTI = exti;
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

    //Interupt for buttons bound to pins px0
    // #[interrupt(resources = [ITM, EXTI, PB0])]
    // fn EXTI0() {
    //     let stim = &mut resources.ITM.stim[0];
    //     iprintln!(stim, "Button was clicked!");
    //     resources.PB0.clear_interrupt_pending_bit(&mut resources.EXTI)
    // }

    //Interupt for buttons bound to pins px1
    // #[interrupt(resources = [ITM, EXTI, PB1])]
    // fn EXTI1() {
    //     let stim = &mut resources.ITM.stim[0];
    //     iprintln!(stim, "Button was clicked!");
    //     resources.PB0.clear_interrupt_pending_bit(&mut resources.EXTI)
    // }

    //Interupt for buttons bound to pins px2
    // #[interrupt(resources = [ITM, EXTI, PB2])]
    // fn EXTI2() {
    //     let stim = &mut resources.ITM.stim[0];
    //     iprintln!(stim, "Button was clicked!");
    //     resources.PB0.clear_interrupt_pending_bit(&mut resources.EXTI)
    // }
    
    #[interrupt(resources = [ITM, EXTI, PC7, PC8, PC9])]
    fn EXTI9_5() {
        let stim = &mut resources.ITM.stim[0];
        iprintln!(stim, "Button was clicked!");
        resources.PC7.clear_interrupt_pending_bit(&mut resources.EXTI);
        resources.PC8.clear_interrupt_pending_bit(&mut resources.EXTI);
        resources.PC9.clear_interrupt_pending_bit(&mut resources.EXTI);

    }


    extern "C" {
        fn EXTI1();
    }
};