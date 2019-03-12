// Values for ACD1_IN1, PA1
// 2.5v 3000
// 1.5v 1850
// 0.5v 615

use core::marker::Unsize;

extern crate stm32f4xx_hal as hal;
use hal::stm32::{ADC1, DMA2, RCC, TIM2, GPIOA, GPIOB, GPIOC};

use cast::u16;
use hal::prelude::*;
use static_ref::Static;

use crate::dma::{self, CircBuffer, Dma2Stream0};
use crate::channel::Channel;
use crate::pwm::Pwm;

/// Input channel associated to ADC1
#[derive(Clone, Copy, Debug)]
pub enum AdcChannel {
    /// ADC1_IN0 = PA0
    _0 = 0,
    /// ADC1_IN1 = PA1
    _1 = 1,
    /// ADC1_IN2 = PA2 (Unimplemented: conflicts with USB USART2_TX)
    /// _2 = 2,
    /// ADC1_IN3 = PA3 (Unimplemented: conflicts with USB USART2_RX)
    /// _3 = 3,
    /// ADC1_IN4 = PA4
    _4 = 4,
    /// ADC1_IN5 = PA5
    _5 = 5,
    /// ADC1_IN6 = PA6
    _6 = 6,
    /// ADC1_IN7 = PA7
    _7 = 7,
    /// ADC1_IN8 = PB0
    _8 = 8,
    /// ADC1_IN9 = PB1
    _9 = 9,
    /// ADC1_IN10 = PC0
    _10 = 10,
    /// ADC1_IN11 = PC1
    _11 = 11,
    /// ADC1_IN12 = PC2
    _12 = 12,
    /// ADC1_IN13 = PC3
    _13 = 13,
    /// ADC1_IN14 = PC4
    _14 = 14,
    /// ADC1_IN15 = PC5
    _15 = 15,
}

pub struct Adc<'a>(pub &'a ADC1);

impl<'a> Adc<'a> {
    /// Enables the ADC input
    pub fn enable_input(
        &self,
        input: AdcChannel,
        sq: u8,
        gpioa: &GPIOA,
        gpiob: &GPIOB,
        gpioc: &GPIOC,
    ) {
        let adc1 = self.0;
    
        // RM0368 11.12.9
        unsafe {
            match sq {
                1 => adc1.sqr3.modify(|_, w| w.sq1().bits(input as u8)),
                2 => adc1.sqr3.modify(|_, w| w.sq2().bits(input as u8)),
                3 => adc1.sqr3.modify(|_, w| w.sq3().bits(input as u8)),
                4 => adc1.sqr3.modify(|_, w| w.sq4().bits(input as u8)),
                5 => adc1.sqr3.modify(|_, w| w.sq5().bits(input as u8)),
                6 => adc1.sqr3.modify(|_, w| w.sq6().bits(input as u8)),
                7 => adc1.sqr2.modify(|_, w| w.sq7().bits(input as u8)),
                8 => adc1.sqr2.modify(|_, w| w.sq8().bits(input as u8)),
                9 => adc1.sqr2.modify(|_, w| w.sq9().bits(input as u8)),
                10 => adc1.sqr2.modify(|_, w| w.sq10().bits(input as u8)),
                11 => adc1.sqr2.modify(|_, w| w.sq11().bits(input as u8)),
                12 => adc1.sqr2.modify(|_, w| w.sq12().bits(input as u8)),
                13 => adc1.sqr1.modify(|_, w| w.sq13().bits(input as u8)),
                14 => adc1.sqr1.modify(|_, w| w.sq14().bits(input as u8)),
                15 => adc1.sqr1.modify(|_, w| w.sq15().bits(input as u8)),
                16 => adc1.sqr1.modify(|_, w| w.sq16().bits(input as u8)),
                _ => panic!("invalid sequence register"),
            }
        }

        // Use as many conversions as maximum channel sequence number
        let l = adc1.sqr1.read().l().bits();
        if l < sq {
            adc1.sqr1
                .modify(|_, w| w.l().bits(sq.wrapping_sub(1)) );
        }
        // Set pins as analog input
        match input {
            AdcChannel::_0 => {
                gpioa.afrl.modify(|_, w| w.afrl0().bits(0));
                gpioa.moder.modify(|_, w| w.moder0().bits(0b11));
                gpioa.pupdr.modify(|_, w| unsafe { w.pupdr0().bits(0b00) });
            }
            AdcChannel::_1 => {
                gpioa.afrl.modify(|_, w| w.afrl1().bits(0));
                gpioa.moder.modify(|_, w| w.moder1().bits(0b11));
                gpioa.pupdr.modify(|_, w| unsafe { w.pupdr1().bits(0b00) });
            }
            // AdcChannel::_2 => {
            //     unimplemented!();
            // }
            // AdcChannel::_3 => {
            //     unimplemented!();
            // }
            AdcChannel::_4 => {
                gpioa.afrl.modify(|_, w| w.afrl4().bits(0));
                gpioa.moder.modify(|_, w| w.moder4().bits(0b11));
                gpioa.pupdr.modify(|_, w| unsafe { w.pupdr4().bits(0b00) });
            }
            AdcChannel::_5 => {
                gpioa.afrl.modify(|_, w| w.afrl5().bits(0));
                gpioa.moder.modify(|_, w| w.moder5().bits(0b11));
                gpioa.pupdr.modify(|_, w| unsafe { w.pupdr5().bits(0b00) });
            }
            AdcChannel::_6 => {
                gpioa.afrl.modify(|_, w| w.afrl6().bits(0));
                gpioa.moder.modify(|_, w| w.moder6().bits(0b11));
                gpioa.pupdr.modify(|_, w| unsafe { w.pupdr6().bits(0b00) });
            }
            AdcChannel::_7 => {
                gpioa.afrl.modify(|_, w| w.afrl7().bits(0));
                gpioa.moder.modify(|_, w| w.moder7().bits(0b11));
                gpioa.pupdr.modify(|_, w| unsafe { w.pupdr7().bits(0b00) });
            }
            AdcChannel::_8 => unsafe {
                gpiob.afrl.modify(|_, w| w.afrl0().bits(0));
                gpiob.moder.modify(|_, w| w.moder0().bits(0b11));
                gpiob.pupdr.modify(|_, w| w.pupdr0().bits(0b00));
            },
            AdcChannel::_9 => unsafe {
                gpiob.afrl.modify(|_, w| w.afrl1().bits(0));
                gpiob.moder.modify(|_, w| w.moder1().bits(0b11));
                gpiob.pupdr.modify(|_, w| w.pupdr1().bits(0b00));
            },
            AdcChannel::_10 => unsafe {
                gpioc.afrl.modify(|_, w| w.afrl0().bits(0));
                gpioc.moder.modify(|_, w| w.moder0().bits(0b11));
                gpioc.pupdr.modify(|_, w| w.pupdr0().bits(0b00));
            },
            AdcChannel::_11 => unsafe {
                gpioc.afrl.modify(|_, w| w.afrl1().bits(0));
                gpioc.moder.modify(|_, w| w.moder1().bits(0b11));
                gpioc.pupdr.modify(|_, w| w.pupdr1().bits(0b00));
            },
            AdcChannel::_12 => unsafe {
                gpioc.afrl.modify(|_, w| w.afrl2().bits(0));
                gpioc.moder.modify(|_, w| w.moder2().bits(0b11));
                gpioc.pupdr.modify(|_, w| w.pupdr2().bits(0b00));
            },
            AdcChannel::_13 => unsafe {
                gpioc.afrl.modify(|_, w| w.afrl3().bits(0));
                gpioc.moder.modify(|_, w| w.moder3().bits(0b11));
                gpioc.pupdr.modify(|_, w| w.pupdr3().bits(0b00));
            },
            AdcChannel::_14 => unsafe {
                gpioc.afrl.modify(|_, w| w.afrl4().bits(0));
                gpioc.moder.modify(|_, w| w.moder4().bits(0b11));
                gpioc.pupdr.modify(|_, w| w.pupdr4().bits(0b00));
            },
            AdcChannel::_15 => unsafe {
                gpioc.afrl.modify(|_, w| w.afrl5().bits(0));
                gpioc.moder.modify(|_, w| w.moder5().bits(0b11));
                gpioc.pupdr.modify(|_, w| w.pupdr5().bits(0b00));
            },
        }
    }

    pub fn init(&self, dma2: &DMA2, rcc: &RCC) {
        let adc1 = self.0;
        // enable ADC1, DMA2, GPIOA, TIM2
        rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
        rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());
        rcc.ahb1enr.modify(|_, w| w.gpiocen().set_bit());
        rcc.ahb1enr.modify(|_, w| w.dma2en().set_bit());
        rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());
        rcc.apb2enr.modify(|_, w| w.adc1en().set_bit());

        // RM0368 11.12.5
        // Sample time: 55.5 + 12.5 = 68 cycles
        adc1.smpr2.modify(|_, w| unsafe { w.smpx_x().bits(0) });

        // ADC1
        // chsel: Channel 0 (RM0368 9.3.3 Table 27)
        // pl: Medium priority
        // msize: Memory size = 16 bits
        // psize: Peripheral size = 16 bits
        // minc: Memory increment mode enabled
        // pinc: Peripheral increment mode disabled
        // circ: Circular mode enabled
        // dir: Transfer from peripheral to memory
        // htie: Half transfer interrupt enabled
        // tceie: Transfer complete interrupt enabled
        // en: Disabled
        dma2.s0cr.write(|w| unsafe {
            w.chsel()
                .bits(0)
                .pl()
                .bits(0b01)
                .msize()
                .bits(0b01)
                .psize()
                .bits(0b01)
                .minc()
                .set_bit()
                .circ()
                .set_bit()
                .pinc()
                .clear_bit()
                .dir()
                .bits(0)
                .htie()
                .set_bit()
                .tcie()
                .set_bit()
                .en()
                .clear_bit()
        });
        // RM0368 11.12.3
        // exten: Conversion on external trigger rising edge
        // extsel: Timer 2 CC2 event
        // align: Right alignment
        // dma: DMA mode enabled
        // dds: DMA requests are issued as long as data are converted and DMA=1
        // cont: Single conversion mode
        // adon: Disable ADC conversion
        adc1.cr2.write(|w| unsafe {
            w.exten()
                .bits(0b01)
                .extsel()
                .bits(0b011) // T2C2
                .align()
                .clear_bit()
                .dma()
                .set_bit()
                .dds()
                .set_bit()
                .cont()
                .clear_bit()
                .adon()
                .clear_bit()
        });
        // RM0368 11.3.8 and 11.12.2
        // scan: Scan mode enabled
        adc1.cr1.write(|w| w.scan().set_bit());        
    }


    /// Disables the ADC
    pub fn disable(&self) {
        self.0.cr2.modify(|_, w| w.adon().clear_bit());
    }

    /// Enables the ADC
    pub fn enable(&self) {
        self.0.cr2.modify(|_, w| w.adon().set_bit());
    }

    /// Starts an analog to digital conversion that will be periodically
    /// triggered by the channel 2 of TIM2
    ///
    /// The conversions will be stored in the circular `buffer`
    pub fn start<B>(
        &self,
        buffer: & dma::CircBuffer<B, dma::Dma2Stream0>,
        dma2: &DMA2,
        pwm: &mut Pwm<TIM2>,
    ) -> Result<(), dma::Error>
    where
        B: Unsize<[u16]>,
    {
        let adc1 = self.0;

        if dma2.s0cr.read().en().bit_is_set() {
            return Err(dma::Error::InUse);
        }

        pwm.disable(Channel::_2);
        pwm.set_duty(Channel::_2, 1);

        let buffer: &[u16] = &buffer.lock()[0];

        dma2.s0ndtr
            .write(|w| w.ndt().bits(u16(buffer.len() * 2).unwrap()) );

        dma2.s0par
            .write(|w| unsafe { w.bits(&adc1.dr as *const _ as u32) });

        dma2.s0m0ar
            .write(|w| unsafe { w.bits(buffer.as_ptr() as u32) });

        dma2.s0cr.modify(|_, w| w.en().set_bit());
        pwm.enable(Channel::_2);
        pwm.set_duty(Channel::_2, 2);
        Ok(())
    }
}