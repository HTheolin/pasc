// main.rs
// Author: PASC
// Version: 0.1.0
// Description:
// Date: 05/03-18
#![no_main]
#![no_std]
#![feature(unsize)]
#![feature(never_type)]

extern crate panic_halt;

use cortex_m::{asm, iprintln};

extern crate embedded_hal;
extern crate stm32f4xx_hal as hal;

#[macro_use(block)]
extern crate nb;

use embedded_hal::blocking::i2c::{WriteRead};
use crate::hal::prelude::*;

use hal::stm32::{ITM, DMA2, EXTI, I2C1, SPI1};
use hal::gpio::gpioc::{PC3, PC6, PC7, PC8, PC9};
use hal::gpio::gpiob::{PB0, PB1, PB2};
use hal::gpio::{Output, PushPull, Speed};
use hal::gpio::{Input, PullDown, ExtiPin, Edge};
use hal::time::{Hertz, KiloHertz};
use hal::spi::Spi;
use hal::i2c::{I2c, PinScl, PinSda};
use hal::timer::Timer;
use hal::prelude::_embedded_hal_timer_CountDown as CountDown;
// use hal::prelude::_embedded_hal_blocking_i2c_WriteRead as WriteRead;


use rtfm::{app, Instant};

mod adc;
mod pwm;
mod frequency;
mod time;
mod dma;
mod channel;
mod button;
mod lcd;
mod pcd8544;
mod pcd8544_spi;
mod demo;
mod font;
mod lis3dh;

use channel::Channel;
use dma::{CircBuffer, Dma2Stream0};
use pcd8544::{Pcd8544, Pcd8544Spi};

//use button::{BUTTON, PB0};
const FREQUENCY: time::Hertz = time::Hertz(100);
const LCDFREQUENCY: time::Hertz = time::Hertz(1000);
const ADCFREQUENCY: time::Hertz = time::Hertz(8);
const I2CFREQUENCY: KiloHertz = KiloHertz(1);
const SPIFREQUENCY: Hertz = Hertz(100);
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
    static mut BPC13: button::PC13 = ();
    static mut BPC7: button::PC7  = ();
    static mut BPC8: button::PC8  = ();
    static mut BPC9: button::PC9  = ();
    // static mut I2C1: I2c<I2C1, (hal::gpio::gpiob::PB6<hal::gpio::Alternate<hal::gpio::AF4>>, hal::gpio::gpiob::PB7<hal::gpio::Alternate<hal::gpio::AF4>>)> = ();
    static mut SPI: Spi<SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5>>)> = ();
    static mut LCD: Pcd8544Spi<PB0<Output<PushPull>>, PC3<Output<PushPull>>> = ();
     // static mut BPB0: button::PB0  = ();
    // static mut BPB1: button::PB1  = ();
    // static mut BPB0: button::PB2  = ();
    static mut I2C1: I2C1 = ();
    
    static mut BUFFER: CircBuffer<'static, [u16; N], Dma2Stream0> = CircBuffer::new([[0; N]; 2]);
    
    #[init(resources = [BUFFER], schedule = [trace])]
    fn init() {
        let rcc = device.RCC;
        let dma2 = device.DMA2;
        let adc1 = device.ADC1;
        let tim1 = device.TIM1;
        let tim2 = device.TIM2;
        let tim3 = device.TIM3;
        let tim5 = device.TIM5;
        let spi1 = device.SPI1;
        let exti = device.EXTI;
        let syscfg = device.SYSCFG;
        let i2c1 = device.I2C1;

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

       //Enable pwm for driving the lcd contrast, tim3 channel 1 = PC6
        let mut pwm = pwm::Pwm(&tim3);
        let c = &Channel::_1;
        pwm.init(
            LCDFREQUENCY.invert(),
            *c,
            None,
            &device.GPIOA,
            &device.GPIOB,
            &device.GPIOC,
            &rcc,
        );
        pwm.set_duty(*c, pwm.get_max_duty() / 2);
        pwm.enable(*c);

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


        let button = button::BPC13;
        button.init(&device.GPIOC, &rcc, &syscfg, &exti, Edge::FALLING);
 
        button::BPC7.init(&device.GPIOC, &rcc, &syscfg, &exti, Edge::FALLING);
        button::BPC8.init(&device.GPIOC, &rcc, &syscfg, &exti, Edge::FALLING);
        button::BPC9.init(&device.GPIOC, &rcc, &syscfg, &exti, Edge::FALLING);

        // // button::BPB0.init(&device.GPIOB, &rcc, &syscfg, &exti, Edge::FALLING);
        // button::BPB1.init(&device.GPIOB, &rcc, &syscfg, &exti, Edge::FALLING);
        // button::BPB2.init(&device.GPIOB, &rcc, &syscfg, &exti, Edge::FALLING);

        

        // Init the I2C peripheral
        // let i2c = i2c::I2c(&i2c1);
        // i2c.init(&device.GPIOA, &device.GPIOB, &rcc);
        // i2c.enable();

        // i2c.start(lis3dh::ADDRESS);
        // let lis3dh = lis3dh::Lis3dh::new(&i2c, lis3dh::ADDRESS);
        // let stim = &mut core.ITM.stim[0];
        // let mut data = [0b0111];
        // let res = i2c.write(lis3dh::ADDRESS, &mut [lis3dh::LIS3DH_REG_CTRL1])
        //                             .map_err(lis3dh::Error::I2C)
        //                             .and(Ok(data[0]));



        // let pinscl = gpiob.pb6.into_alternate_af4();
        // let pinscl = pinscl.set_speed(Speed::VeryHigh);
        // let pinscl = pinscl.set_open_drain();
        // let pinscl = pinscl.internal_pull_up(false);
        // let pinsda = gpiob.pb7.into_alternate_af4();
        // let pinsda = pinsda.set_speed(Speed::VeryHigh);
        // let pinsda = pinsda.set_open_drain();
        // let pinsda = pinsda.internal_pull_up(false);

        // let mut i2c = I2c::i2c1(
        //     i2c1,
        //     (pinscl, pinsda),
        //     I2CFREQUENCY,
        //     clocks,
        // );

        let stim = &mut core.ITM.stim[0];
        iprintln!(stim, "i2c" );
      
        lis3dh::init(&i2c1, &device.GPIOB, &rcc);

        // iprintln!(stim, "I Am: {} ", rx_buffer[0]);

        // let stim = &mut core.ITM.stim[0];
        // let mut data = [0];
        // let res = i2c.write_read(lis3dh::ADDRESS, &[lis3dh::LIS3DH_REG_WHOAMI], &mut data)
        //                             .map_err(lis3dh::Error::I2C)
        //                             .and(Ok(data[0]));
        // let mut data = 0x07;
        // let res = i2c.write(lis3dh::ADDRESS, &[lis3dh::LIS3DH_REG_CTRL1, data]);
        // iprintln!(stim, "I am: {}", data[0]);


        let rcc = rcc.constrain();
        let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(16.mhz()).pclk2(16.mhz()).freeze();
        
        
        let gpiob = device.GPIOB.split();
 
        let mut timer = Timer::tim5(tim5, SPIFREQUENCY, clocks);

        let (mut spi, mut pcd8544) = lcd::init(&mut timer, device.GPIOA, gpiob.pb0.into_push_pull_output(), device.GPIOC, clocks, spi1);
        
        // schedule.run_demo(Instant::now() + 16_000_000.cycles()).unwrap();
        demo::demo(&mut pcd8544, &mut spi);



        // let gpiob = device.GPIOB.split();
 
        // let mut timer = Timer::tim5(tim5, SPIFREQUENCY, clocks);
        // timer.start(hal::time::Hertz(1000));
        // block!(timer.wait()).unwrap();
        lis3dh::enable(&i2c1);
        //   iprintln!(stim, "Enable");
        while lis3dh::start(&i2c1).is_err() {};

          iprintln!(stim, "Start");
        // while lis3dh::stop(&i2c1).is_err() {};
        // iprintln!(stim, "Stop");

        let mut rx_buffer = [0; 2];
        while lis3dh::write(&i2c1, lis3dh::LIS3DH_REG_WHOAMI as u8).is_err() {}
        let mut RX_BUFFER_SIZE: usize = 2;
        for i in 0..RX_BUFFER_SIZE {
            rx_buffer[i] = loop {
                if i == RX_BUFFER_SIZE - 1 {
                    // Do not ACK the last byte received and send STOP
                    if let Ok(byte) = lis3dh::read_nack(&i2c1) {
                        break byte;
                    }
                } else {
                    // ACK the byte after receiving
                    if let Ok(byte) = lis3dh::read_ack(&i2c1) {
                        break byte;
                    }
                }
            }
        }
        
        while lis3dh::stop(&i2c1).is_err()  {};
        iprintln!(stim, "Values are {} ", rx_buffer[0]);

        //Enable adc after splash screen!
        adc.enable();
        adc.start(resources.BUFFER, &dma2, &mut pwm2).unwrap();
        
        // schedule.trace(Instant::now() + (16_000_000).cycles()).unwrap();
        BPC7 = button::BPC7;
        BPC8 = button::BPC8;
        BPC9 = button::BPC9;
        // PB0 = timstart;
        // BP1 = timinc;
        // BP2 = timres;
        I2C1 = i2c1;
        SPI = spi;
        LCD = pcd8544;
        ITM = core.ITM;
        DMA2 = dma2;
        EXTI = exti;
        BPC13 = button::BPC13;
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

        schedule.trace(Instant::now() + (16_000_000).cycles()).unwrap();
    }

    #[interrupt(resources = [ITM, BUFFER, DMA2, LCD, SPI])]
    fn DMA2_STREAM0() {
        resources.ITM.lock(|itm| {
            let stim = &mut itm.stim[0];
        });
        
        match resources.BUFFER.read(resources.DMA2, |x| {
                let buf: [u16; N] = x.clone();
                buf
            }) {
                Err(_) => cortex_m::asm::bkpt(),
                Ok(b) => {
                    resources.LCD.clear(&mut resources.SPI);
                    resources.LCD.print_char(&mut resources.SPI, 'o' as u8);
                    resources.LCD.print_char(&mut resources.SPI, 'k' as u8);
                    // iprintln!(stim, "{}, {}", b[0], b[1]);
                }
            }
    }

    //Interupt for buttons bound to pins px0
    // #[interrupt(resources = [ITM, EXTI, BPB0])]
    // fn EXTI0() {
    //     let stim = &mut resources.ITM.stim[0];
    //     iprintln!(stim, "Button was clicked!");
    //     resources.BPB0.clearpending(&mut resources.EXTI)
    // }

    //Interupt for buttons bound to pins px1
    // #[interrupt(resources = [ITM, EXTI, BPB1])]
    // fn EXTI1() {
    //     let stim = &mut resources.ITM.stim[0];
    //     iprintln!(stim, "Button was clicked!");
    //     resources.BPB0.clear_pending(&mut resources.EXTI)
    // }

    //Interupt for buttons bound to pins px2
    // #[interrupt(resources = [ITM, EXTI, BPB2])]
    // fn EXTI2() {
    //     let stim = &mut resources.ITM.stim[0];
    //     iprintln!(stim, "Button was clicked!");
    //     resources.BPB0.clear_pending(&mut resources.EXTI)
    // }
    
    #[interrupt(resources = [ITM, EXTI, BPC7, BPC8, BPC9, LCD, SPI])]
    fn EXTI9_5() {
        resources.ITM.lock(|itm| {
            let stim = &mut itm.stim[0];
            iprintln!(stim, "Button was clicked!");
        });
        
        resources.BPC7.clear_pending(&mut resources.EXTI);
        resources.BPC8.clear_pending(&mut resources.EXTI);
        resources.BPC9.clear_pending(&mut resources.EXTI);
        resources.LCD.set_position(&mut resources.SPI, 1, 2);
        resources.LCD.print(&mut resources.SPI, "Button was clicked!");
    }


    // #[interrupt(resources = [ITM, EXTI, BPC13])]
    // fn EXTI15_10() {
    //     resources.ITM.lock(|itm| {
    //         let stim = &mut itm.stim[0];
    //         iprintln!(stim, "Button was clicked!");
    //     });
        
    //     resources.BPC13.clear_pending(&mut resources.EXTI);
    // }

    extern "C" {
        fn EXTI1();
        fn EXTI2();
    }
};
