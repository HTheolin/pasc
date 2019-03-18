#![deny(unsafe_code)]
#![no_std]

use core::marker::{Unsize};
use hal::spi::{Spi, NoMiso, Pins, PinSck, PinMiso, PinMosi};
use hal::rcc::Clocks;
use hal::prelude::*;
use hal::stm32::{GPIOA, GPIOB, GPIOC, TIM5, SPI1};
use hal::gpio::gpiob::{PB0};
use hal::gpio::gpioc::{PC0, PC1, PC2, PC3, PC4};
use hal::gpio::{Output, PushPull};
use hal::timer::Timer;
use hal::prelude::_embedded_hal_digital_OutputPin as OutputPin;
use hal::prelude::_embedded_hal_blocking_spi_Write as Write;

use embedded_hal::spi;
use crate::pcd8544::{Pcd8544Spi, Pcd8544};
use crate::pcd8544_spi;
use crate::demo;

pub fn init(timer: &mut Timer<TIM5>, gpioa: GPIOA, gpiob: GPIOB, gpioc: GPIOC, clocks: Clocks, spi1: SPI1) -> 
(Spi<SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5>>)>,
Pcd8544Spi<PB0<Output<PushPull>>, PC3<Output<PushPull>>>) {
    let gpioa = gpioa.split();
    let sck = gpioa.pa5.into_alternate_af5();
    let miso = NoMiso;
    let mosi = gpioa.pa7.into_alternate_af5();
    let spi_mode = spi::Mode {
        phase: spi::Phase::CaptureOnFirstTransition,
        polarity: spi::Polarity::IdleLow,
    };

    let mut spi = Spi::spi1(
        spi1,
        (sck, miso, mosi),
        spi_mode,
        4_000_000.hz(),
        clocks,
    );
        
    let gpiob = gpiob.split();
    let gpioc = gpioc.split();

    let dc: PB0<Output<PushPull>> = gpiob.pb0.into_push_pull_output().into();
    let cs: PC3<Output<PushPull>> = gpioc.pc3.into_push_pull_output().into();
    let mut rst: PC4<Output<PushPull>>  = gpioc.pc4.into_push_pull_output().into();

    // let dc: PC2<Output<PushPull>> = gpioc.pc2.into_push_pull_output().into();
    // let cs: PC0<Output<PushPull>> = gpioc.pc0.into_push_pull_output().into();
    // let mut rst: PC1<Output<PushPull>>  = gpioc.pc1.into_push_pull_output().into();
    
    let mut pcd8544 = Pcd8544Spi::new(dc, cs, &mut rst, timer);
    
    pcd8544.init(&mut spi);
    (spi, pcd8544)
}

