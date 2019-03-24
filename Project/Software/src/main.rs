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
use hal::gpio::gpioa::{PA5, PA7};
use hal::gpio::gpioc::{PC0, PC2, PC3, PC6, PC7, PC8, PC9};
use hal::gpio::gpiob::{PB0, PB1, PB2};
use hal::gpio::{Output, PushPull, Speed, Input, PullDown, ExtiPin, Edge, Alternate, AF5};
use hal::time::{Hertz, KiloHertz, MilliSeconds};
use hal::i2c::{I2c, PinScl, PinSda};
use hal::timer::Timer;
use hal::prelude::_embedded_hal_timer_CountDown as CountDown;
// use hal::prelude::_embedded_hal_blocking_i2c_WriteRead as WriteRead;

use rtfm::{app, Instant};

mod adc;
mod button;
mod channel;
mod demo;
mod dma;
mod font;
mod frequency;
mod lcd;
mod lis3dh;
mod pcd8544;
mod pcd8544_spi;
mod pwm;
mod temp;
mod time;
mod pedometer;

use channel::Channel;
use dma::{CircBuffer, Dma2Stream0};
use lis3dh::Accelerometer;
use pedometer::Pedometer;

const CLOCK: u32 = 64_000_000;
//use button::{BUTTON, PB0};
const FREQUENCY: time::Hertz = time::Hertz(100);
const LCDFREQUENCY: time::Hertz = time::Hertz(1000);
const ADCFREQUENCY: time::Hertz = time::Hertz(8);
const I2CFREQUENCY: KiloHertz = KiloHertz(1);
const SPIFREQUENCY: Hertz = Hertz(100);

const SECOND: u32 = CLOCK ;
const MILLISECOND: u32 = CLOCK / 1000;
const N: usize = 2;
const SAMPLESIZE: usize = 50;
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

    static mut BPB5: button::PB5 = ();

    // Toggle these to change board
    static mut BPC7: button::PC7  = ();
    static mut BPC8: button::PC8  = ();
    static mut BPC9: button::PC9  = ();
    // static mut BPB0: button::PB0  = ();
    // static mut BPB1: button::PB1  = ();
    // static mut BPB2: button::PB2  = ();
    
    static mut LIS3DH: Accelerometer = (); 
    static mut PEDOMETER: Pedometer = ();
    static mut LCD: lcd::Lcd = ();

    static mut BUFFER: CircBuffer<'static, [u16; N], Dma2Stream0> = CircBuffer::new([[0; N]; 2]);
    static mut STEPTIMEOUT: bool = true;
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

        //Enable pwm for driving the lcd contrast, tim3 channel 1 = PC6 (Henrik), = PA6 (Simon)
        // let mut pwm = pwm::Pwm(&tim3);
        // let c = &Channel::_1;
        // pwm.init(
        //     LCDFREQUENCY.invert(),
        //     *c,
        //     None,
        //     &device.GPIOA,
        //     &device.GPIOB,
        //     &device.GPIOC,
        //     &rcc,
        // );
        // pwm.set_duty(*c, pwm.get_max_duty() / 2);
        // pwm.enable(*c);
        // iprintln!(stim, "{:?}", pwm.get_max_duty()/2);

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
        button.init(&device.GPIOC, &rcc, &syscfg, &exti, Edge::FALLING, false);
 
        button::BPB5.init(&device.GPIOB, &rcc, &syscfg, &exti, Edge::FALLING, false);
        
        // Toggle commeting on these to change board
        // button::BPC7.init(&device.GPIOC, &rcc, &syscfg, &exti, Edge::FALLING, false);
        // button::BPC8.init(&device.GPIOC, &rcc, &syscfg, &exti, Edge::FALLING, false);
        // button::BPC9.init(&device.GPIOC, &rcc, &syscfg, &exti, Edge::FALLING, false);
        button::BPB0.init(&device.GPIOB, &rcc, &syscfg, &exti, Edge::FALLING, false);
        button::BPB1.init(&device.GPIOB, &rcc, &syscfg, &exti, Edge::FALLING, false);
        button::BPB2.init(&device.GPIOB, &rcc, &syscfg, &exti, Edge::FALLING, false);

        let stim = &mut core.ITM.stim[0];
        // Initiates the i2c bus at 100khz

        lis3dh::init(&i2c1, &device.GPIOB, &rcc);
        let mut accelerometer = lis3dh::Accelerometer::new(i2c1, lis3dh::Range::LIS3DH_RANGE_2_G);
        // Get clock for timer to enable a delay in the lcd startup sequence
        let rcc = rcc.constrain();
        let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(16.mhz()).pclk2(16.mhz()).freeze();
        
        let mut timer = Timer::tim5(tim5, SPIFREQUENCY, clocks);


        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();

        // LCD.
        // To change between boards, comment/uncomment these lines.
        // Also change the macro call in lcd.rs!

        // // Simon PCB LCD.
        // let sce  = gpioc.pc0.into_push_pull_output().into();
        // let rst  = gpioc.pc1.into_push_pull_output().into();
        // let dc   = gpioc.pc2.into_push_pull_output().into();
        // let mosi = gpioa.pa7.into_alternate_af5();
        // let sck  = gpioa.pa5.into_alternate_af5();

        // Henrik PCB LCD.
        let sce  = gpioc.pc5.into_push_pull_output().into();
        let rst  = gpioc.pc4.into_push_pull_output().into();
        let dc   = gpiob.pb0.into_push_pull_output().into();
        let mosi = gpioa.pa7.into_alternate_af5();
        let sck  = gpioa.pa5.into_alternate_af5();

        let lcd = lcd::Lcd::init(&mut timer, sce, rst, dc, mosi, sck, clocks, spi1);
       
        let mut buffer = [0; 1];
        accelerometer.who_am_i(&mut buffer).unwrap();
        iprintln!(stim, "I AM {} ", buffer[0]);
        accelerometer.setup();
        accelerometer.set_datarate(lis3dh::Datarate::LIS3DH_DATARATE_400_HZ);
        accelerometer.set_range(lis3dh::Range::LIS3DH_RANGE_2_G);
        iprintln!(stim, "Wrote registers");
        accelerometer.set_click_interrupt(1, 1, 200, 0, 100);
      
        let pedometer = Pedometer::new(0.4);
        //Enable adc after splash screen!
        adc.enable();
        adc.start(resources.BUFFER, &dma2, &mut pwm2).unwrap();
        
        BPB5 = button::BPB5;
        // Toggle commeting on these to change board
        BPC7 = button::BPC7;
        BPC8 = button::BPC8;
        BPC9 = button::BPC9;

        // BPB0 = button::BPB0;
        // BPB1 = button::BPB1;
        // BPB2 = button::BPB2;
        LIS3DH = accelerometer;
        PEDOMETER = pedometer;
        LCD = lcd;
        ITM = core.ITM;
        DMA2 = dma2;
        EXTI = exti;
        BPC13 = button::BPC13;
    }

    #[idle(spawn = [trace])]
    fn idle() -> ! {
        spawn.trace();
        loop {
            asm::wfi();
        }
    }

    /// Periodic task for your pleasure
    #[task(resources = [ITM, LCD], schedule = [trace])]
    fn trace() {
        let stim = &mut resources.ITM.stim[0];
        resources.LCD.update();
        schedule.trace(Instant::now() + (4*SECOND).cycles()).unwrap();
    }

    // Direct Memory Access buffer filled by ADC interrupts.
    #[interrupt(resources = [BUFFER, DMA2, LCD, ITM])]
    fn DMA2_STREAM0() {
        let stim = &mut resources.ITM.stim[0];
        match resources.BUFFER.read(resources.DMA2, |x| {
                let buf: [u16; N] = x.clone();
                buf
        }) {
            Err(_) => cortex_m::asm::bkpt(),
            Ok(b) => {
                //iprintln!(stim, "{:?}", b[0]);
                let temp = temp::to_celsius(b[0]);
                //iprintln!(stim, "{:?}", temp);
                resources.LCD.temp_write(temp as f32);
            }
        }
    }

    // /// Interupt for buttons bound to pins px0
    // #[interrupt(resources = [ITM, EXTI, BPB0, LCD])]
    // fn EXTI0() {
    //     let stim = &mut resources.ITM.stim[0];
    //     let lcd = &mut resources.LCD;
    //     iprintln!(stim, "Button was clicked!");
    //     lcd.step_add();
    //     resources.BPB0.clear_pending(&mut resources.EXTI)
    // }

    // /// Interupt for buttons bound to pins px1
    // #[interrupt(resources = [ITM, EXTI, BPB1, LCD])]
    // fn EXTI1() {
    //     let stim = &mut resources.ITM.stim[0];
    //     let lcd = &mut resources.LCD;
    //     iprintln!(stim, "Button was clicked!");
    //     lcd.step_reset();
    //     resources.BPB1.clear_pending(&mut resources.EXTI)
    // }

    // /// Interupt for buttons bound to pins px2
    // #[interrupt(resources = [ITM, EXTI, BPB2, LCD])]
    // fn EXTI2() {
    //     let stim = &mut resources.ITM.stim[0];
    //     let lcd = &mut resources.LCD;
    //     iprintln!(stim, "Button was clicked!");
    //     lcd.write_line(2, "Button 3!");
    //     resources.BPB2.clear_pending(&mut resources.EXTI)
    // }

    /// Interrupt for pins 5-9
    // #[interrupt(resources = [ITM, EXTI, I2C1, BPB5, BPC7, BPC8, BPC9, LCD, SPI])]
    #[interrupt(resources = [ITM, EXTI, LIS3DH, BPB5, LCD, STEPTIMEOUT, PEDOMETER], schedule = [clear_timeout])]
    fn EXTI9_5() {
        let stim = &mut resources.ITM.stim[0];

        let mut data = [0; 6];
        resources.LIS3DH.read_accelerometer(&mut data).unwrap();
        let x_g = resources.LIS3DH.axis().x_g();
        let y_g = resources.LIS3DH.axis().y_g();
        let z_g = resources.LIS3DH.axis().z_g();
        resources.PEDOMETER.set_samples(x_g, y_g, z_g);
        if resources.PEDOMETER.get_samples() >= pedometer::SAMPLELIMIT {
            resources.PEDOMETER.calc_max();
            resources.PEDOMETER.calc_min();                             
            resources.PEDOMETER.calc_threshold();
            iprintln!(stim, "Max value: {}", resources.PEDOMETER.get_max());
            iprintln!(stim, "Min value: {}", resources.PEDOMETER.get_min());
            iprintln!(stim, "Threshold is: {}", resources.PEDOMETER.get_threshold());
            resources.PEDOMETER.reset_samples();
        } else {
            resources.PEDOMETER.increment_sample();
        }

        if *resources.STEPTIMEOUT {
            *resources.STEPTIMEOUT = false;
            iprintln!(stim, "Accelerometer values: x: {}, y: {}, z: {}", x_g, y_g, z_g);
            let step = match resources.PEDOMETER.get_direction() {
                pedometer::Direction::X => x_g,
                pedometer::Direction::Y => y_g,
                pedometer::Direction::Z => z_g,
            };

            if resources.PEDOMETER.is_step(step) {
                resources.PEDOMETER.add_step();
                resources.LCD.set_steps(resources.PEDOMETER.get_steps());
            }
            
            schedule.clear_timeout(Instant::now() + (200*MILLISECOND).cycles()).unwrap();
        }
        
        resources.BPB5.clear_pending(&mut resources.EXTI);
        // resources.BPC7.clear_pending(&mut resources.EXTI);
        // resources.BPC8.clear_pending(&mut resources.EXTI);
        // resources.BPC9.clear_pending(&mut resources.EXTI);
    }
    
    #[task(resources = [STEPTIMEOUT])]
    fn clear_timeout() {
        *resources.STEPTIMEOUT = true;
    }
    // /// Interrupt for PC13 user btn on the nucleo board.
    // #[interrupt(resources = [ITM, EXTI, BPC13])]
    // fn EXTI15_10() {
    //     resources.ITM.lock(|itm| {
    //         let stim = &mut itm.stim[0];
    //         iprintln!(stim, "Button was clicked!");
    //     });
        
    //     resources.BPC13.clear_pending(&mut resources.EXTI);
    // }

    extern "C" {
        fn EXTI4();
    }
};
