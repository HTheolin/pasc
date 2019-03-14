#![deny(unsafe_code)]
#![no_std]

use hal::delay::Delay;
use hal::spi::{Spi, NoMiso};
use hal::prelude::*;
use hal::stm32::{GPIOA, GPIOB, GPIOC, SYST};
use hal::gpio::gpiob::{PB0};
use hal::gpio::gpioc::{PC0, PC1, PC2, PC3, PC4};
use hal::gpio::{Output, PushPull};

use embedded_hal::spi;
use crate::pcd8544::{Pcd8544Spi, Pcd8544};
use crate::demo;

pub struct Lcd<'a> {
    pcd8544: &'a mut Pcd8544,
}

pub fn init(syst: SYST, gpioa: GPIOA, gpiob: GPIOB, gpioc: GPIOC) {
    let gpioa = gpioa.split();
    let sck = gpioa.pa5.into_alternate_af5();
    let miso = NoMiso;
    let mosi = gpioa.pa7.into_alternate_af5();
    let spi_mode = spi::Mode {
        phase: spi::Phase::CaptureOnFirstTransition,
        polarity: spi::Polarity::IdleLow,
    };

    let p = hal::stm32::Peripherals::take().unwrap();

    let rcc = p.RCC.constrain();
    let syst = syst;
    // 16 MHz (default, all clocks)
    // let clocks = rcc.cfgr.freeze();
    let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).pclk2(34.mhz()).freeze();

    let mut delay = Delay::new(syst, clocks);
    
    let spi = Spi::spi1(
        p.SPI1,
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

    let mut pcd8544 = Pcd8544Spi::new(spi, dc, cs, &mut rst, &mut delay);
    
    demo::demo(&mut pcd8544);
}
