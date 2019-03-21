#![deny(unsafe_code)]
#![no_std]

use hal::spi::{Spi, NoMiso, PinSck, PinMiso, PinMosi};
use hal::rcc::Clocks;
use hal::prelude::*;
use hal::stm32::{GPIOA, GPIOC, TIM5, SPI1};
use hal::gpio::gpioa::{PA5, PA7};
use hal::gpio::gpiob::{PB0};
use hal::gpio::gpioc::{PC0, PC1, PC2, PC3, PC4};
use hal::gpio::{Output, PushPull, Alternate, AF5};
use hal::timer::Timer;

use embedded_hal::spi;
use crate::pcd8544::{Pcd8544Spi, Pcd8544};



pub struct LcdData {
    temp: i32, // Temperature: Celsius
}

impl LcdData{
    pub fn new(temp: i32) -> LcdData {
        LcdData {
            temp: temp,
        }
    }
    
    pub fn temp_write(&mut self, temp: i32) {
        self.temp = temp;
    }

    pub fn temp_read(&self) -> i32 {
        self.temp
    }
}

pub fn init(timer: &mut Timer<TIM5>, gpioa: GPIOA, dc: PB0<Output<PushPull>>, gpioc: GPIOC, clocks: Clocks, spi1: SPI1) 
    -> 
    (Spi<SPI1, (PA5<hal::gpio::Alternate<AF5>>, 
                NoMiso, 
                PA7<Alternate<AF5>>)>,
    Pcd8544Spi<PB0<Output<PushPull>>, PC3<Output<PushPull>>>) 
    {
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
        
    // let gpiob = gpiob.split();
    let gpioc = gpioc.split();

    let dc: PB0<Output<PushPull>> = dc.into();
    let cs: PC3<Output<PushPull>> = gpioc.pc3.into_push_pull_output().into();
    let mut rst: PC4<Output<PushPull>>  = gpioc.pc4.into_push_pull_output().into();

    // let dc: PC2<Output<PushPull>> = gpioc.pc2.into_push_pull_output().into();
    // let cs: PC0<Output<PushPull>> = gpioc.pc0.into_push_pull_output().into();
    // let mut rst: PC1<Output<PushPull>>  = gpioc.pc1.into_push_pull_output().into();
    
    let mut pcd8544 = Pcd8544Spi::new(dc, cs, &mut rst, timer);
    
    pcd8544.init(&mut spi);
    (spi, pcd8544)
}


pub fn init_alt(timer: &mut Timer<TIM5>, gpioa: GPIOA, gpioc: GPIOC, clocks: Clocks, spi1: SPI1) 
    -> 
    (Spi<SPI1, (PA5<Alternate<AF5>>, 
                NoMiso, 
                PA7<Alternate<AF5>>)>,
    Pcd8544Spi<PC2<Output<PushPull>>, PC0<Output<PushPull>>>) 
    {
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
        1000000.hz(),
        clocks,
    );
        
    // let gpiob = gpiob.split();
    let gpioc = gpioc.split();

    let dc: PC2<Output<PushPull>> = gpioc.pc2.into_push_pull_output().into();
    let cs: PC0<Output<PushPull>> = gpioc.pc0.into_push_pull_output().into();
    let mut rst: PC1<Output<PushPull>>  = gpioc.pc1.into_push_pull_output().into();
    
    let mut pcd8544 = Pcd8544Spi::new(dc, cs, &mut rst, timer);
    
    pcd8544.init(&mut spi);
    (spi, pcd8544)
}

