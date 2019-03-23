// lcd.rs
// Author: PASC
// Version: 0.1.0
// Description: LCD object. Contains LcdData structure that contains
//              data that is periodically updated on the display and
//              written to by tasks.
//              
//              Largely implemented by macro due to different PCBs and
//              pin configurations.
// Date: 23/03-18

#![deny(unsafe_code)]
#![no_std]

use hal::spi::{Spi, NoMiso};
use hal::rcc::Clocks;
use hal::prelude::*;
use hal::stm32::{TIM5, SPI1};
use hal::gpio::gpioa::{PA5, PA7};
use hal::gpio::gpiob::{PB0};
use hal::gpio::gpioc::{PC0, PC1, PC2, PC4, PC5};
use hal::gpio::{Output, PushPull, Alternate, AF5};
use hal::timer::Timer;

use numtoa::NumToA; // Integers to characters.
use ryu;            // Floats to characters.

use embedded_hal::spi;
use crate::pcd8544::{Pcd8544Spi, Pcd8544};

// Change macro call at bottom of this file to change board.
//      Simon PCB (SPCB): DC is PC2, SCE is PC0.
//      Henrik PCB (HPCB): DC is PB0, SCE is PC5.
macro_rules! implement_lcd {
    ( $sce:ty, $rst:ty, $dc:ty ) => {
        pub struct Lcd {
            data: LcdData,
            spi: Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>,
            device: Pcd8544Spi<$dc, $sce>,
            rst: $rst,
        }
        
        impl Lcd {
            pub fn init(
                timer: &mut Timer<TIM5>,
                sce: $sce,
                mut rst: $rst,
                dc: $dc,
                mosi: PA7<Alternate<AF5>>, // SPI MOSI pin
                sck: PA5<Alternate<AF5>>,  // SPI SCK pin
                clocks: Clocks,
                spi1: SPI1
            ) -> Self {
                let miso = NoMiso;

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

                // Reset LCD to known state.
                rst.set_low();
                timer.start(hal::time::Hertz(100));
                block!(timer.wait()).unwrap();
                rst.set_high();

                // PCD8544 SPI interface.
                let mut pcd8544 = Pcd8544Spi::new(dc, sce);

                // Init device.
                pcd8544.init(&mut spi);

                let lcd = Lcd {
                    data: LcdData::new(),
                    spi: spi,
                    device: pcd8544,
                    rst: rst,
                };

                return lcd;
            }
        }
    }
}

impl Lcd {
    pub fn update(&mut self) {
        // Latest values already displayed.
        if self.data.new_data == false {
            return;
        }

        self.device.clear(&mut self.spi);

        // Temperature.
        let mut buffer = ryu::Buffer::new();
        let temp = buffer.format(self.data.temp);
        let temp_suffix: &[u8] = &[b' ', 248, b'C']; // 248 is extended ASCII degree sign.
        self.device.set_position(&mut self.spi, 0u8, 0u8);
        self.device.print(&mut self.spi, temp);
        self.device.print_bytes(&mut self.spi, temp_suffix);

        // Step counter.
        let mut buffer: [u8; 6] = [0u8; 6];
        let steps: &[u8] = self.data.step.numtoa(10, &mut buffer); // Base 10.
        let step_suffix: &[u8] = " steps!".as_bytes();
        self.device.set_position(&mut self.spi, 0u8, 2u8);
        self.device.print_bytes(&mut self.spi, steps);
        self.device.print_bytes(&mut self.spi, step_suffix);
                
        self.data.new_data = false;
    }

    // Temperature
    pub fn temp_write(&mut self, temp: f32) {
        if self.data.temp != temp {
            self.data.temp = temp;
            self.data.new_data = true;
        }
    }

    // Public reads are only needed in debug.
    pub fn temp_read(&self) -> f32 {
        self.data.temp
    }

    // Step counter.
    pub fn step_add(&mut self) {
        self.data.step += 1;
        self.data.new_data = true;
    }

    pub fn step_reset(&mut self) {
        self.data.step = 0;
        self.data.new_data = true;
    }

    // Pulses per minute.
    // TODO ...

    pub fn write_line(&mut self, row: u8, line: &str) {
        self.device.set_position(&mut self.spi, 0u8, row);
        self.device.print(&mut self.spi, line);
    }
}

pub struct LcdData {
    // Flag is set when there is new data since last display update().
    new_data: bool,

    // Data.
    temp: f32, // Temperature: Celsius
    step: u32, // Step counter.
    pulse: u32, // Pulse, beats per minute.
}

impl LcdData{
    pub fn new() -> Self {
        LcdData {
            new_data: true,
            temp: 0f32,
            step: 0u32,
            pulse: 0u32,
        }
    }
}

// HPCB
implement_lcd!(PC5<Output<PushPull>>, PC4<Output<PushPull>>, PB0<Output<PushPull>>);
// SPCB
// implement_lcd!(PC0<Output<PushPull>>, PC1<Output<PushPull>>, PC2<Output<PushPull>>);
