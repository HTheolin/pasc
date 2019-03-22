use core::marker::{Unsize};
use core::any::{Any, TypeId};

// use _hal::prelude::*;
use hal::stm32::{DMA1, TIM1, TIM2, TIM3, TIM4, RCC, GPIOA, GPIOB, GPIOC};

use static_ref::Static;
use cast::{u16, u32};

use crate::dma::{self, Buffer, Dma1Stream2};
use crate::frequency::*;
use crate::channel::Channel;
/// PWM driver
pub struct Pwm<'a, T>(pub &'a T)
where
    T: 'a;

macro_rules! impl_Pwm {
    ($TIM:ident, $APB:ident) => {
        impl<'a> Pwm<'a, $TIM>
        {
            /// Initializes the PWM module
            pub fn init<P>(
                &self,
                period: P,
                channel: Channel,
                dma1: Option<&DMA1>,
                gpioa: &GPIOA,
                gpiob: &GPIOB,
                gpioc: &GPIOC,
                rcc: &RCC,
            ) where
                P: Into<$APB::Ticks>,
            {
                self._init(period.into(), channel, dma1, gpioa, gpiob, gpioc, rcc)
            }

            fn _init(
                &self,
                period: $APB::Ticks,
                channel: Channel,
                dma1: Option<&DMA1>,
                gpioa: &GPIOA,
                gpiob: &GPIOB,
                gpioc: &GPIOC,
                rcc: &RCC,
            ) {
                let tim = self.0;

                // enable AFIO, (DMA1), TIMx and GPIOx
                if dma1.is_some() {
                    rcc.ahb1enr.modify(|_, w| w.dma1en().set_bit());
                }

                if tim.type_id() == TypeId::of::<TIM1>() {
                    rcc.apb2enr.modify(|_, w| w.tim1en().set_bit());
                } else if tim.type_id() == TypeId::of::<TIM2>() {
                    rcc.apb1enr.modify(|_, w| w.tim2en().set_bit());
                } else if tim.type_id() == TypeId::of::<TIM3>() {
                    rcc.apb1enr.modify(|_, w| w.tim3en().set_bit());
                } else if tim.type_id() == TypeId::of::<TIM4>() {
                    rcc.apb1enr.modify(|_, w| w.tim4en().set_bit());
                }

                rcc.ahb1enr.modify(|_, w| {
                    if tim.type_id() == TypeId::of::<TIM1>() {
                        w.gpioaen().set_bit()
                    } else if tim.type_id() == TypeId::of::<TIM2>() {
                        match channel {
                            Channel::_1 =>  w.gpioaen().set_bit(),
                            Channel::_2 =>  w.gpioaen().set_bit(),
                            Channel::_3 =>  w.gpioben().set_bit(),
                            Channel::_4 =>  unimplemented!()
                        }
                    } else if tim.type_id() == TypeId::of::<TIM3>() {
                        match channel {
                            Channel::_1 =>  w.gpioaen().set_bit(),
                            Channel::_2 =>  w.gpiocen().set_bit(),
                            Channel::_3 =>  w.gpioben().set_bit(),
                            Channel::_4 =>  w.gpioben().set_bit(),
                        }
                    } else if tim.type_id() == TypeId::of::<TIM4>() {
                        w.gpioben().set_bit()
                    } else {
                        unreachable!()
                    }
                });
                
                // See datasheet DM00115249 Table 9. Alternate function mapping
                if tim.type_id() == TypeId::of::<TIM1>() {
                    // CH1 = PA8 = alternate push-pull
                    // CH2 = PA9 = alternate push-pull
                    // CH3 = PA10 = alternate push-pull
                    // CH4 = PA11 = alternate push-pull
                    match channel {
                        Channel::_1 => {
                            gpioa.afrh.modify(|_, w|  w.afrh8().bits(1));
                            gpioa.moder.modify(|_, w| w.moder8().bits(2));
                        }
                        Channel::_2 => {
                            gpioa.afrh.modify(|_, w|  w.afrh9().bits(1));
                            gpioa.moder.modify(|_, w| w.moder9().bits(2));
                        }
                        Channel::_3 => {
                            gpioa.afrh.modify(|_, w|  w.afrh10().bits(1));
                            gpioa.moder.modify(|_, w| w.moder10().bits(2));
                        }
                        Channel::_4 => {
                            gpioa.afrh.modify(|_, w|  w.afrh11().bits(1));
                            gpioa.moder.modify(|_, w| w.moder11().bits(2));
                        }
                    }
                } else if tim.type_id() == TypeId::of::<TIM2>() {
                    // CH1 = PA15 = alternate push-pull
                    // CH2 = PA1 = alternate push-pull
                    // CH3 = PB10 = alternate push-pull
                    // CH4 = PA3 = alternate push-pull (Not implemented: conflicts with USB USART2_RX)

                    // See datasheet DM00115249 Table 9. Alternate function mapping
                    match channel {
                        Channel::_1 => {
                            gpioa.afrh.modify(|_, w| w.afrh15().bits(1));
                            gpioa.moder.modify(|_, w| w.moder15().bits(2));
                        }
                        Channel::_2 => {
                            gpioa.afrl.modify(|_, w| w.afrl1().bits(1));
                            gpioa.moder.modify(|_, w| w.moder1().bits(2));
                        }
                        Channel::_3 => {
                            gpiob.afrh.modify(|_, w| w.afrh10().bits(1));
                            gpiob.moder.modify(|_, w| w.moder10().bits(2));
                        }
                        Channel::_4 => {
                            unimplemented!()
                        }
                    }
                } else if tim.type_id() == TypeId::of::<TIM3>() {
                    // CH1 = PC6, PA6 = alternate push-pull
                    // CH2 = PC7 = alternate push-pull
                    // CH3 = PB0 = alternate push-pull
                    // CH4 = PB1 = alternate push-pull
                    match channel {
                        Channel::_1 => {
                            // PA6 (Simon PCB)
                            gpioa.afrl.modify(|_, w| w.afrl6().bits(2));
                            gpioc.moder.modify(|_, w| w.moder6().bits(2));
                            // // PC6 (Henrik PCB)
                            // gpioc.afrl.modify(|_, w| w.afrl6().bits(2));
                            // gpioc.moder.modify(|_, w| w.moder6().bits(2));
                        }
                        Channel::_2 => {
                            gpioc.afrl.modify(|_, w|  w.afrl7().bits(2));
                            gpioc.moder.modify(|_, w| w.moder7().bits(2));
                        }
                        Channel::_3 => {
                            gpiob.afrl.modify(|_, w| w.afrl0().bits(2));
                            gpiob.moder.modify(|_, w| w.moder0().bits(2));
                        }
                        Channel::_4 => {
                            gpiob.afrl.modify(|_, w| w.afrl1().bits(2));
                            gpiob.moder.modify(|_, w| w.moder1().bits(2));
                        }
                    }

                } else if tim.type_id() == TypeId::of::<TIM4>() {
                    // CH1 = PB6 = alternate push-pull
                    // CH2 = PB7 = alternate push-pull
                    // CH3 = PB8 = alternate push-pull
                    // CH4 = PB9 = alternate push-pull
                    match channel {
                        Channel::_1 => {
                            gpiob.afrl.modify(|_, w|  w.afrl6().bits(2));
                            gpiob.moder.modify(|_, w| w.moder6().bits(2));
                        }
                        Channel::_2 => {
                            gpiob.afrl.modify(|_, w|  w.afrl7().bits(2));
                            gpiob.moder.modify(|_, w| w.moder7().bits(2));
                        }
                        Channel::_3 => {
                            gpiob.afrh.modify(|_, w|  w.afrh8().bits(2));
                            gpiob.moder.modify(|_, w| w.moder8().bits(2));
                        }
                        Channel::_4 => {
                            gpiob.afrh.modify(|_, w|  w.afrh9().bits(2));
                            gpiob.moder.modify(|_, w| w.moder9().bits(2));
                        }
                    }
                }

                // PWM mode 1
                match channel {
                    Channel::_1 => {
                        tim.ccmr1_output.modify(|_, w| unsafe {w.oc1pe().set_bit().oc1m().bits(0b110)});
                        tim.ccer.modify(|_, w| {w.cc1p().clear_bit()});
                    }
                    Channel::_2 => {
                        tim.ccmr1_output.modify(|_, w| unsafe {w.oc2pe().set_bit().oc2m().bits(0b110)});
                        tim.ccer.modify(|_, w| {w.cc2p().clear_bit()});
                    }
                    Channel::_3 => {
                        tim.ccmr2_output.modify(|_, w| unsafe {w.oc3pe().set_bit().oc3m().bits(0b110)});
                        tim.ccer.modify(|_, w| {w.cc3p().clear_bit()});
                    }
                    Channel::_4 => {
                        if tim.type_id() == TypeId::of::<TIM2>() {
                            unimplemented!()
                        }
                        tim.ccmr2_output.modify(|_, w| unsafe {w.oc4pe().set_bit().oc4m().bits(0b110)});
                        tim.ccer.modify(|_, w| {w.cc4p().clear_bit()});
                    }
                }

                self._set_period(period);

                if let Some(dma1) = dma1 {
                    //  Update DMA request enable
                    tim.dier.modify(|_, w| w.ude().set_bit());

                    if tim.type_id() == TypeId::of::<TIM3>() {
                        // TIM3_CH4/UP
                        // chsel: Channel 5 (RM0368 9.3.3 Table 27)
                        // pl: Medium priority
                        // msize: Memory size = 8 bits
                        // psize: Peripheral size = 16 bits
                        // minc: Memory increment mode enabled
                        // pinc: Peripheral increment mode disabled
                        // circ: Circular mode disabled
                        // dir: Transfer from memory to peripheral
                        // tcie: Transfer complete interrupt enabled
                        // en: Disabled
                        dma1.s2cr.write(|w| unsafe {
                            w.chsel()
                                .bits(5)
                                .pl()
                                .bits(0b01)
                                .msize()
                                .bits(0b00)
                                .psize()
                                .bits(0b01)
                                .minc()
                                .set_bit()
                                .circ()
                                .set_bit()
                                .pinc()
                                .clear_bit()
                                .dir()
                                .bits(1)
                                .tcie()
                                .set_bit()
                                .en()
                                .clear_bit()
                        });
                    } else {
                        unimplemented!()
                    }
                }

                tim.cr1.write(|w| 
                    w.cms()
                        .bits(0b00)
                        .dir()
                        .bit(false)
                        .opm()
                        .bit(false)
                        .cen()
                        .set_bit()
                );
            }

            fn _set_period(&self, period: $APB::Ticks) {
                let period = period.0;

                let psc = u16((period - 1) / (1 << 16)).unwrap();
                self.0.psc.write(|w| unsafe{w.psc().bits(psc)});

                let arr = u32(period / u32(psc + 1));
                self.0.arr.write(|w| unsafe{w.bits(arr)});
            }

            /// Uses `buffer` to continuously change the duty cycle on every period
            pub fn set_duties<B>(
                &self,
                dma1: &DMA1,
                channel: Channel,
                buffer: &Static<Buffer<B, Dma1Stream2>>,
            ) -> ::core::result::Result<(), dma::Error>
            where
                B: Unsize<[u8]>,
            {
                let tim3 = self.0;

                if tim3.type_id() == TypeId::of::<TIM3>() {
                    if dma1.s2cr.read().en().bit_is_set() {
                        return Err(dma::Error::InUse);
                    }

                    let buffer: &[u8] = buffer.lock();

                    dma1.s2ndtr.write(|w| w.ndt().bits(u16(buffer.len()).unwrap()) );
                    dma1.s2par.write(|w| unsafe {
                        match channel {
                            Channel::_1 => w.bits(&tim3.ccr1 as *const _ as u32),
                            Channel::_2 => w.bits(&tim3.ccr2 as *const _ as u32),
                            Channel::_3 => w.bits(&tim3.ccr3 as *const _ as u32),
                            Channel::_4 => w.bits(&tim3.ccr4 as *const _ as u32),
                        }
                    });
                    dma1.s2m0ar.write(|w| unsafe { w.bits(buffer.as_ptr() as u32) });
                    dma1.s2cr.modify(|_, w| w.en().set_bit());

                    Ok(())

                } else {
                    unimplemented!()
                }
            }
        }
    }
}

macro_rules! impl_halPwm {
    ($TIM:ident, $APB:ident) => {
        impl<'a> embedded_hal::Pwm for Pwm<'a, $TIM>
        {
            type Channel = Channel;
            type Duty = u32;
            type Time = $APB::Ticks;

            fn get_duty(&self, channel: Channel) -> u32 {
                match channel {
                    Channel::_1 => u32(self.0.ccr1.read().ccr1().bits()),
                    Channel::_2 => u32(self.0.ccr2.read().ccr2().bits()),
                    Channel::_3 => u32(self.0.ccr3.read().ccr3().bits()),
                    Channel::_4 => u32(self.0.ccr4.read().ccr4().bits()),
                }
            }

            fn disable(&mut self, channel: Channel) {
                match channel {
                    Channel::_1 => self.0.ccer.modify(|_, w| w.cc1e().clear_bit()),
                    Channel::_2 => self.0.ccer.modify(|_, w| w.cc2e().clear_bit()),
                    Channel::_3 => self.0.ccer.modify(|_, w| w.cc3e().clear_bit()),
                    Channel::_4 => self.0.ccer.modify(|_, w| w.cc4e().clear_bit()),
                }
            }

            fn enable(&mut self, channel: Channel) {
                match channel {
                    Channel::_1 => self.0.ccer.modify(|_, w| w.cc1e().set_bit()),
                    Channel::_2 => self.0.ccer.modify(|_, w| w.cc2e().set_bit()),
                    Channel::_3 => self.0.ccer.modify(|_, w| w.cc3e().set_bit()),
                    Channel::_4 => self.0.ccer.modify(|_, w| w.cc4e().set_bit()),
                }
            }

            fn get_max_duty(&self) -> u32 {
                self.0.arr.read().bits()
            }

            fn get_period(&self) -> $APB::Ticks {
                $APB::Ticks(u32(self.0.psc.read().bits() * self.0.arr.read().bits()))
            }

            fn set_duty(&mut self, channel: Channel, duty: u32) {
                //let duty : u16 = u16(duty).unwrap();
                match channel {
                    Channel::_1 => self.0.ccr1.write(|w|
                        w.ccr1().bits(duty)),
                    Channel::_2 => self.0.ccr2.write(|w|
                        w.ccr2().bits(duty)),
                    Channel::_3 => self.0.ccr3.write(|w|
                        w.ccr3().bits(duty)),
                    Channel::_4 => self.0.ccr4.write(|w|
                        w.ccr4().bits(duty)),
                }
            }

            fn set_period<P>(&mut self, period: P)
            where
                P: Into<$APB::Ticks>,
            {
                self._set_period(period.into())
            }
        }
    }
}

impl_Pwm!(TIM1, apb2);
impl_Pwm!(TIM2, apb1);
impl_halPwm!(TIM2, apb1);
impl_Pwm!(TIM3, apb1);
impl_halPwm!(TIM3, apb1);
impl_Pwm!(TIM4, apb1);
impl_halPwm!(TIM4, apb1);


// TIM1 is 16 bit instead of 32
impl<'a> embedded_hal::Pwm for Pwm<'a, TIM1> {
    type Channel = Channel;
    type Time = apb2::Ticks;
    type Duty = u16;

    fn disable(&mut self, channel: Channel) {
        match channel {
            Channel::_1 => self.0.ccer.modify(|_, w| w.cc1e().clear_bit()),
            Channel::_2 => self.0.ccer.modify(|_, w| w.cc2e().clear_bit()),
            Channel::_3 => self.0.ccer.modify(|_, w| w.cc3e().clear_bit()),
            Channel::_4 => self.0.ccer.modify(|_, w| w.cc4e().clear_bit()),
        }
    }

    fn enable(&mut self, channel: Channel) {
        self.0.bdtr.modify(|_, w|  w.moe().set_bit());
        match channel {
            Channel::_1 => self.0.ccer.modify(|_, w| w.cc1e().set_bit()),
            Channel::_2 => self.0.ccer.modify(|_, w| w.cc2e().set_bit()),
            Channel::_3 => self.0.ccer.modify(|_, w| w.cc3e().set_bit()),
            Channel::_4 => self.0.ccer.modify(|_, w| w.cc4e().set_bit()),
        }
    }

    fn get_duty(&self, channel: Channel) -> u16 {
        match channel {
            Channel::_1 => self.0.ccr1.read().ccr1().bits(),
            Channel::_2 => self.0.ccr2.read().ccr2().bits(),
            Channel::_3 => self.0.ccr3.read().ccr3().bits(),
            Channel::_4 => self.0.ccr4.read().ccr4().bits(),
        }
    }

    fn get_max_duty(&self) -> u16 {
        self.0.arr.read().arr().bits()
    }

    fn get_period(&self) -> apb2::Ticks {
        apb2::Ticks(u32(self.0.psc.read().bits() * self.0.arr.read().bits()))
    }

    fn set_duty(&mut self, channel: Channel, duty: u16) {
        match channel {
            Channel::_1 => self.0.ccr1.write(|w| unsafe { w.ccr1().bits(duty) }),
            Channel::_2 => self.0.ccr2.write(|w| unsafe { w.ccr2().bits(duty) }),
            Channel::_3 => self.0.ccr3.write(|w| unsafe { w.ccr3().bits(duty) }),
            Channel::_4 => self.0.ccr4.write(|w| unsafe { w.ccr4().bits(duty) }),
        }
    }

    fn set_period<P>(&mut self, period: P)
    where
        P: Into<apb2::Ticks>,
    {
        self._set_period(period.into())
    }
}
