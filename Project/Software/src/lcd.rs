#![deny(unsafe_code)]
#![no_std]

use core::marker::{Unsize};

use hal::delay::Delay;
use hal::spi::{Spi, PinSck, PinMosi, NoMiso};
use hal::prelude::*;
use hal::stm32::{GPIOA, GPIOB, GPIOC, SPI1, SYST, FLASH};
use hal::gpio::gpioa::{PA5, PA7};
use hal::gpio::gpiob::{PB0};
use hal::gpio::gpioc::{PC3, PC4};
use hal::gpio::{Output, PushPull};
use hal::rcc;

use embedded_hal::spi;
use embedded_hal::digital::OutputPin;

extern crate pcd8544_hal;
use pcd8544_hal::Pcd8544Spi;
use crate::spi::Spi as Spi_impl;

pub struct Lcd<>;

impl<'a> Lcd<> {
    pub fn init_super(self, syst: SYST, spi1: &SPI1, gpioa: GPIOA, gpiob: GPIOB, gpioc: GPIOC){
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
        // other pins for PCD8544
        let dc: PB0<Output<PushPull>> = gpiob.pb0.into_push_pull_output().into();
        let cs: PC3<Output<PushPull>> = gpioc.pc3.into_push_pull_output().into();
        let mut rst: PC4<Output<PushPull>>  = gpioc.pc4.into_push_pull_output().into();

        let mut pcd8544 = Pcd8544Spi::new(spi, dc, cs, &mut rst, &mut delay);


       // pcd8544_hal::demo::demo(&mut pcd8544);

    }  
}