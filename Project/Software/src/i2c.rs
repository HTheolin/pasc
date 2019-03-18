//! Inter-Integrated Circuit (I2C)
//!
//! You can use the I2C interface with these instances
//!
//! # I2C1
//! - SCL = PB8
//! - SDA = PB9
//!
//! # I2C2
//! - SCL = PB10
//! - SDA = PB3
//!
//! # I2C3
//! - SCL = PA8
//! - SDA = PB4

use core::any::{Any, TypeId};
use core::ptr;

use nb;
use hal::stm32::{I2C1, I2C2, I2C3, GPIOA, GPIOB, RCC};

use crate::frequency::*;
use crate::time::*;
/// I2C result
pub type Result<T> = ::core::result::Result<T, nb::Error<Error>>;

/// I2C error
#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Timeout occurred, SCL remained LOW for 25 ms
    Timeout,
    /// Bus error
    BusError,
    #[doc(hidden)] _Extensible,
}

/// Serial Peripheral Interface
pub struct I2c<'a, T>(pub &'a T)
where
    T: 'a;

/// I2C interface
macro_rules! impl_I2c {
    ($S:ident) => {
        impl<'a> I2c<'a, $S> {
            /// Initializes the SPI
            pub fn init(&self, gpioa: &GPIOA, gpiob: &GPIOB, rcc: &RCC) {
                let i2c = self.0;
                if i2c.type_id() == TypeId::of::<I2C1>() {
                    // # I2C1
                    // - SCL = PB6
                    // - SDA = PB7

                    // Enable I2C1, GPIOB
                    rcc.apb1enr.modify(|_, w| w.i2c1en().set_bit());
                    rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());
                    // DM00102166 - Alternate function, Table 9
                    gpiob.afrl.modify(|_, w| unsafe {
                        w.afrl6().bits(4)
                        .afrl7().bits(4)});
                    // RM0368 8.3 Table 23
                    // Highest output speed
                    gpiob.ospeedr.modify(|_, w| unsafe {
                        w.ospeedr6().bits(0b11)
                        .ospeedr7().bits(0b11)});
                    // Alternate function mode
                    gpiob.moder.modify(|_, w| unsafe {
                        w.moder6().bits(2)
                        .moder7().bits(2)});
                    // Alternate function open drain
                    gpiob.otyper.modify(|_, w|
                        w.ot6().set_bit()
                        .ot7().set_bit());
                    // Floating
                    gpiob.pupdr.modify(|_, w| unsafe {
                        w.pupdr6().bits(0)
                        .pupdr7().bits(0)});

                } else if i2c.type_id() == TypeId::of::<I2C2>() {
                    // # I2C2
                    // - SCL = PB10
                    // - SDA = PB3

                    // Enable I2C2, GPIOB
                    rcc.apb1enr.modify(|_, w| w.i2c2en().set_bit());
                    rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());
                    // DM00102166 - Alternate function, Table 9
                    gpiob.afrl.modify(|_, w| unsafe {w.afrl3().bits(9)});
                    gpiob.afrh.modify(|_, w| unsafe {w.afrh10().bits(4)});
                    // RM0368 8.3 Table 23
                    // Highest output speed
                    gpiob.ospeedr.modify(|_, w| unsafe {
                        w.ospeedr3().bits(0b11)
                        .ospeedr10().bits(0b11)});
                    // Alternate function mode
                    gpiob.moder.modify(|_, w| unsafe {
                        w.moder3().bits(2)
                        .moder10().bits(2)});
                    // Alternate function open drain
                    gpiob.otyper.modify(|_, w|
                        w.ot3().set_bit()
                        .ot10().set_bit());
                    // Floating
                    gpiob.pupdr.modify(|_, w| unsafe {
                        w.pupdr3().bits(0)
                        .pupdr10().bits(0)});

                } else if i2c.type_id() == TypeId::of::<I2C3>() {
                    // # I2C3
                    // - SCL = PA8
                    // - SDA = PB4

                    // Enable I2C1, GPIOA, GPIOB
                    rcc.apb1enr.modify(|_, w| w.i2c3en().set_bit());
                    rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit().gpioben().set_bit());
                    // DM00102166 - Alternate function, Table 9
                    gpioa.afrh.modify(|_, w| w.afrh8().bits(4));
                    gpiob.afrl.modify(|_, w| unsafe {w.afrl4().bits(9)});
                    // RM0368 8.3 Table 23
                    // Highest output speed
                    gpioa.ospeedr.modify(|_, w| w.ospeedr8().bits(0b11));
                    gpiob.ospeedr.modify(|_, w| unsafe {w.ospeedr4().bits(0b11)});
                    // Alternate function mode
                    gpioa.moder.modify(|_, w| w.moder8().bits(2));
                    gpiob.moder.modify(|_, w| unsafe {w.moder4().bits(2)});
                    // Alternate function open drain
                    gpioa.otyper.modify(|_, w| w.ot8().set_bit());
                    gpiob.otyper.modify(|_, w| w.ot4().set_bit());
                    // Floating
                    gpioa.pupdr.modify(|_, w| unsafe {w.pupdr8().bits(0)});
                    gpiob.pupdr.modify(|_, w| unsafe {w.pupdr4().bits(0)});
                }

                self.disable();

                // Peripheral bus frequency in MHz
                let pclk1_mhz: u32 = apb1::Ticks::from(Microseconds(1)).into();
                let pclk1_hz: u32 = apb1::Ticks::from(Seconds(1)).into();

                i2c.cr1.write(|w|  w.swrst().set_bit());
                i2c.cr1.write(|w| unsafe{ w.bits(0) });

                i2c.cr2.modify(|_,w| unsafe { w.freq().bits(pclk1_mhz as u8) });

                // Use 100_000 Hz baud rate
                let mut result: u16 = (pclk1_hz / (100_000/2)) as u16;
                if result == 0 {
                    result = 1;
                }
                // RM0368 18.6.8
                i2c.ccr.modify(|_,w| unsafe {
                    w.f_s().clear_bit() // Standard mode I2C
                    .duty().clear_bit() // Fast mode duty cycle: t_low/t_high = 2
                    .ccr().bits(result)
                });
                i2c.trise.modify(|_,w| unsafe { w.trise().bits((pclk1_mhz+1) as u8)});
            }

            /// Disables the I2C bus
            pub fn disable(&self) {
                self.0.cr1.modify(|_, w| w.pe().clear_bit())
            }

            /// Enables the I2C bus
            pub fn enable(&self) {
                self.0.cr1.modify(|_, w| w.pe().set_bit())
            }

            /// Send a (RE)START condition. TODO: Non-blocking
            pub fn start(&self, address: u8)  -> Result<()> {
                let i2c = self.0;
                if i2c.sr2.read().msl().bit_is_set() {
                    // Already in master mode, this is RESTART

                    if i2c.sr1.read().tx_e().bit_is_clear() {
                    // Wait for tx to empty if not ACK failed.
                        if i2c.sr1.read().af().bit_is_clear() {
                            // No acknowledge failure
                            return Err(nb::Error::WouldBlock);
                        }
                    }
                    // If we got NACK and tx empty, use ACK pulling:
                    i2c.sr1.modify(|_,w| w.af().clear_bit());
                }
                // Enable ACK
                i2c.cr1.modify(|_,w| w.ack().set_bit());
                // Send START condition
                i2c.cr1.modify(|_, w| w.start().set_bit());
                // Wait for repeated start generation
                while i2c.sr1.read().sb().bit_is_clear() {}
                unsafe {
                    ptr::write_volatile(&i2c.dr as *const _ as *mut u8, address);
                }
                // Wait for end of address transmission
                while i2c.sr1.read().addr().bit_is_clear() {
                    if i2c.sr1.read().af().bit_is_set() {
                        return Err(nb::Error::Other(Error::Timeout));
                    }
                }
                Ok(())
            }

            /// Write a byte
            pub fn write(&self, byte: u8) -> Result<()> {
                let i2c = self.0;
                if i2c.sr1.read().addr().bit_is_set() {
                    // Writing right after the address byte
                    let _sr2 = i2c.sr2.read().bits();
                }
                let sr = i2c.sr1.read();
                if sr.ovr().bit_is_set() {
                    Err(nb::Error::Other(Error::Overrun))
                } else if sr.timeout().bit_is_set() {
                    Err(nb::Error::Other(Error::Timeout))
                } else if sr.berr().bit_is_set() {
                    Err(nb::Error::Other(Error::BusError))
                } else
                if sr.tx_e().bit_is_set() || sr.btf().bit_is_set() {
                    Ok(unsafe {
                        ptr::write_volatile(&i2c.dr as *const _ as *mut u8, byte)
                    })
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }

            /// Read a byte and respond with ACK
            pub fn read_ack(&self) -> Result<u8> {
                let i2c = self.0;
                if i2c.sr1.read().addr().bit_is_set() {
                    // Reading right after the address byte
                    let _sr2 = i2c.sr2.read().bits();
                }
                let sr = i2c.sr1.read();
                if sr.ovr().bit_is_set() {
                    Err(nb::Error::Other(Error::Overrun))
                } else if sr.timeout().bit_is_set() {
                    Err(nb::Error::Other(Error::Timeout))
                } else if sr.berr().bit_is_set() {
                    Err(nb::Error::Other(Error::BusError))
                } else
                if sr.rx_ne().bit_is_set() || sr.btf().bit_is_set() {
                    Ok(unsafe {
                        ptr::read_volatile(&i2c.dr as *const _ as *const u8)
                    })
                } else {
                    Err(nb::Error::WouldBlock)
                }
            }

            /// Read a byte and respond with NACK
            pub fn read_nack(&self)  -> Result<u8> {
                let i2c = self.0;
                // In case a single byte has to be received, the Acknowledge disable
                // is made before ADDR flag is cleared. Disable ACK.
                i2c.cr1.modify(|_,w|  w.ack().clear_bit());

                if i2c.sr1.read().addr().bit_is_set() {
                    // Reading right after the address byte
                    let _sr2 = i2c.sr2.read().bits();
                }
                // Send STOP condition
                i2c.cr1.modify(|_, w| w.stop().set_bit());
                self.read_ack()
            }

            /// Send STOP condition
            pub fn stop(&self) -> Result<()> {
                let i2c = self.0;
                // Disable ACK
                i2c.cr1.modify(|_,w|  w.ack().clear_bit());
                if i2c.sr1.read().addr().bit_is_set() {
                    // Reading right after the address byte
                    let _sr2 = i2c.sr2.read();
                }
                if i2c.sr1.read().tx_e().bit_is_clear() && i2c.sr1.read().btf().bit_is_clear() {
                    return Err(nb::Error::WouldBlock);
                }
                // Send STOP condition
                i2c.cr1.modify(|_, w| w.stop().set_bit());
                Ok(())
            }

        }
    }
}

impl_I2c!(I2C1);
impl_I2c!(I2C2);
impl_I2c!(I2C3);
