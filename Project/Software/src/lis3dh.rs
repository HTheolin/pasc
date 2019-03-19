extern crate embedded_hal;

use nb;
// use hal::blocking::i2c::{WriteRead};
use hal::stm32::{I2C1, GPIOA, GPIOB, GPIOC, RCC};
use hal::i2c::I2c;
use core::ptr;
use hal::prelude::_embedded_hal_blocking_i2c_WriteRead as WriteRead;
use hal::prelude::_embedded_hal_blocking_i2c_Write as Write;
use hal::prelude::_embedded_hal_blocking_i2c_Read as Read;

use crate::frequency::*;
use crate::time::*;

pub const ADDRESS: u8 = 0x18;

pub const LIS3DH_REG_STATUS1: u8 = 0x07;
pub const LIS3DH_REG_OUTADC1_L: u8 = 0x08;
pub const LIS3DH_REG_OUTADC1_H: u8 = 0x09;
pub const LIS3DH_REG_OUTADC2_L: u8 = 0x0A;
pub const LIS3DH_REG_OUTADC2_H: u8 = 0x0B;
pub const LIS3DH_REG_OUTADC3_L: u8 = 0x0C;
pub const LIS3DH_REG_OUTADC3_H: u8 = 0x0D;
pub const LIS3DH_REG_INTCOUNT: u8 = 0x0E;
pub const LIS3DH_REG_WHOAMI: u8 = 0x0F;
pub const LIS3DH_REG_TEMPCFG: u8 = 0x1F;
pub const LIS3DH_REG_CTRL1: u8 = 0x20;
pub const LIS3DH_REG_CTRL2: u8 = 0x21;
pub const LIS3DH_REG_CTRL3: u8 = 0x22;
pub const LIS3DH_REG_CTRL4: u8 = 0x23;
pub const LIS3DH_REG_CTRL5: u8 = 0x24;
pub const LIS3DH_REG_CTRL6: u8 = 0x25;
pub const LIS3DH_REG_REFERENCE: u8 = 0x26;
pub const LIS3DH_REG_STATUS2: u8 = 0x27;
pub const LIS3DH_REG_OUT_X_L: u8 = 0x28;
pub const LIS3DH_REG_OUT_X_H: u8 = 0x29;
pub const LIS3DH_REG_OUT_Y_L: u8 = 0x2A;
pub const LIS3DH_REG_OUT_Y_H: u8 = 0x2B;
pub const LIS3DH_REG_OUT_Z_L: u8 = 0x2C;
pub const LIS3DH_REG_OUT_Z_H: u8 = 0x2D;
pub const LIS3DH_REG_FIFOCTRL: u8 = 0x2E;
pub const LIS3DH_REG_FIFOSRC: u8 = 0x2F;
pub const LIS3DH_REG_INT1CFG: u8 = 0x30;
pub const LIS3DH_REG_INT1SRC: u8 = 0x31;
pub const LIS3DH_REG_INT1THS: u8 = 0x32;
pub const LIS3DH_REG_INT1DUR: u8 = 0x33;
pub const LIS3DH_REG_CLICKCFG: u8 = 0x38;
pub const LIS3DH_REG_CLICKSRC: u8 = 0x39;
pub const LIS3DH_REG_CLICKTHS: u8 = 0x3A;
pub const LIS3DH_REG_TIMELIMIT: u8 = 0x3B;
pub const LIS3DH_REG_TIMELATENCY: u8 = 0x3C;
pub const LIS3DH_REG_TIMEWINDOW: u8 = 0x3D;
pub const LIS3DH_REG_ACTTHS: u8 = 0x3E;
pub const LIS3DH_REG_ACTDUR: u8 = 0x3F;

const RX_BUFFER_SIZE: usize = 1;
/// I2C error
#[derive(Debug)]
pub enum I2CError {
    /// Overrun occurred
    Overrun,
    /// Timeout occurred, SCL remained LOW for 25 ms
    Timeout,
    /// Bus error
    BusError,
    #[doc(hidden)] _Extensible,
    NACK,
}

pub enum Error<E> {
    I2C(E),

}

pub fn read_u8(i2c: &mut I2c<I2C1, (hal::gpio::gpiob::PB6<hal::gpio::Alternate<hal::gpio::AF4>>, hal::gpio::gpiob::PB7<hal::gpio::Alternate<hal::gpio::AF4>>)>, 
                reg: u8) -> Result<u8, hal::i2c::Error> {
        let mut byte: [u8; 1] = [0; 1];

        match i2c.write_read(ADDRESS, &[reg], &mut byte) {
            Ok(_) => Ok(byte[0]),
            Err(e) => Err(e),
        }
    }

pub fn read_bytes(i2c: &mut I2c<I2C1, (hal::gpio::gpiob::PB6<hal::gpio::Alternate<hal::gpio::AF4>>, hal::gpio::gpiob::PB7<hal::gpio::Alternate<hal::gpio::AF4>>)>, 
                reg: u8, buf: &mut [u8]) -> Result<(), hal::i2c::Error> {
    i2c.write_read(ADDRESS, &[reg], buf)
}

pub fn write_u8(i2c: &mut I2c<I2C1, (hal::gpio::gpiob::PB6<hal::gpio::Alternate<hal::gpio::AF4>>, hal::gpio::gpiob::PB7<hal::gpio::Alternate<hal::gpio::AF4>>)>, 
                reg: u8, value: u8) -> Result<(), hal::i2c::Error> {
    i2c.write(ADDRESS, &[reg, value])?;

    Ok(())
}

pub fn init(i2c: &I2C1, gpiob: &GPIOB, rcc: &RCC) {
    // # I2C1
    // - SCL = PB6
    // - SDA = PB7

    // Enable I2C1, GPIOB
    rcc.apb1enr.modify(|_, w| w.i2c1en().set_bit());

    rcc.apb1rstr.modify(|_, w| w.i2c1rst().set_bit());
    rcc.apb1rstr.modify(|_, w| w.i2c1rst().clear_bit());


    rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());
    // DM00102166 - Alternate function, Table 9
    gpiob.afrl.modify(|_, w| 
        w.afrl6().bits(4)
        .afrl7().bits(4));
    // RM0368 8.3 Table 23
    // Highest output speed
    gpiob.ospeedr.modify(|_, w| 
        w.ospeedr6().bits(0b11)
        .ospeedr7().bits(0b11));
    // Alternate function mode
    gpiob.moder.modify(|_, w| 
        w.moder6().bits(2)
        .moder7().bits(2));
    // Alternate function open drain
    gpiob.otyper.modify(|_, w|
        w.ot6().set_bit()
        .ot7().set_bit());
    // Floating
    gpiob.pupdr.modify(|_, w| unsafe {
        w.pupdr6().bits(0)
        .pupdr7().bits(0)});


    disable(i2c);

    // Peripheral bus frequency in MHz
    let pclk1_mhz: u32 = apb1::Ticks::from(Microseconds(1)).into();
    let pclk1_hz: u32 = apb1::Ticks::from(Seconds(1)).into();

    i2c.cr1.write(|w|  w.swrst().set_bit());
    i2c.cr1.write(|w| unsafe{ w.bits(0) });

    i2c.cr2.modify(|_,w| unsafe { w.freq().bits(pclk1_mhz as u8) });

    // Use 100_000 Hz baud rate
    // let mut result: u16 = (pclk1_hz / (100_000 * 2)) as u16;
    let result = {
                let result = (pclk1_hz / (100_000 / 2)) as u16;
                if result < 1 {
                    1
                } else {
                    result
                }
            };
    // RM0368 18.6.8
    i2c.ccr.modify(|_,w| unsafe {
        w.f_s().clear_bit() // Standard mode I2C
        .duty().clear_bit() // Fast mode duty cycle: t_low/t_high = 2
        .ccr().bits(result)
    });
    i2c.trise.modify(|_,w| w.trise().bits((pclk1_mhz + 1) as u8));
}

/// Disables the I2C bus
pub fn disable(i2c: &I2C1) {
    i2c.cr1.modify(|_, w| w.pe().clear_bit())
}

/// Enables the I2C bus
pub fn enable(i2c: &I2C1) {
    i2c.cr1.modify(|_, w| w.pe().set_bit())
}


pub fn start(i2c:  &I2C1) -> Result<(), nb::Error<I2CError> > {
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


    Ok(())
}

pub fn write(i2c: &I2C1, byte: u8) -> Result<(), nb::Error<I2CError>> {
    // Send START condition
    i2c.cr1.modify(|_, w| w.start().set_bit());
    // Wait for repeated start generation
    while i2c.sr1.read().sb().bit_is_clear() {}

    if i2c.sr2.read().busy().bit_is_clear() {
        return Err(nb::Error::WouldBlock);
    }

    i2c.dr.write(|w| unsafe { w.bits(u32::from(ADDRESS) << 1) });
    
    // Wait for end of address transmission
    while i2c.sr1.read().addr().bit_is_clear() {
        if i2c.sr1.read().af().bit_is_set() {
            return Err(nb::Error::Other(I2CError::Timeout));
        }
    }

    while i2c.sr1.read().tx_e().bit_is_clear() {}

    let b = i2c.dr.write(|w| unsafe { w.bits(u32::from(byte)) });

    let sr = i2c.sr1.read();

    if sr.ovr().bit_is_set() {
        Err(nb::Error::Other(I2CError::Overrun))
    } else if sr.timeout().bit_is_set() {
        Err(nb::Error::Other(I2CError::Timeout))
    } else if sr.berr().bit_is_set() {
        Err(nb::Error::Other(I2CError::BusError))
    } else if sr.af().bit_is_set() {
        Err(nb::Error::Other(I2CError::NACK))
    } else if sr.btf().bit_is_clear() {
        Ok(b)
    } else {
        Err(nb::Error::WouldBlock)
    } 
}

/// Read a byte and respond with ACK
pub fn read_ack(i2c: &I2C1) -> Result<u8, nb::Error<I2CError>> {
    // Send START condition
    i2c.cr1.modify(|_, w| w.start().set_bit());
    // Enable ACK
    i2c.cr1.modify(|_,w| w.ack().set_bit());
    // Wait for repeated start generation
    while i2c.sr1.read().sb().bit_is_clear() {}

    while i2c.sr2.read().busy().bit_is_clear() {}
    
    i2c.dr.write(|w| unsafe { w.bits((u32::from(ADDRESS) << 1) + 1) });
    
    // Wait for end of address transmission
    while i2c.sr1.read().addr().bit_is_clear() {
        if i2c.sr1.read().af().bit_is_set() {
            return Err(nb::Error::Other(I2CError::Timeout));
        }
    }

    i2c.sr2.read();
    
    while i2c.sr1.read().rx_ne().bit_is_clear() {}

    if sr.ovr().bit_is_set() {
        Err(nb::Error::Other(I2CError::Overrun))
    } else if sr.timeout().bit_is_set() {
        Err(nb::Error::Other(I2CError::Timeout))
    } else if sr.berr().bit_is_set() {
        Err(nb::Error::Other(I2CError::BusError))
    } else if sr.rx_ne().bit_is_set() || sr.btf().bit_is_set() {
         let value = i2c.dr.read().bits() as u8;
         Ok(value)
    } else {
        Err(nb::Error::WouldBlock)
    }
}


/// Read a byte and respond with NACK
pub fn read_nack(i2c: &I2C1)  -> Result<u8, nb::Error<I2CError>> {
    // In case a single byte has to be received, the Acknowledge disable
    // is made before ADDR flag is cleared. Disable ACK.
    i2c.cr1.modify(|_,w|  w.ack().clear_bit());

    if i2c.sr1.read().addr().bit_is_set() {
        // Reading right after the address byte
        let _sr2 = i2c.sr2.read().bits();
    }

    i2c.cr1.modify(|_, w| w.stop().set_bit());
    read_ack(i2c)
}


/// Send STOP condition
pub fn stop(i2c: &I2C1) -> Result<(), nb::Error<I2CError>> {
    // Disable ACK
    i2c.cr1.modify(|_,w|  w.ack().clear_bit());

    let _sr2 = i2c.sr2.read();
    
    if i2c.sr1.read().tx_e().bit_is_clear() && i2c.sr1.read().btf().bit_is_clear() {
        return Err(nb::Error::WouldBlock);
    }
    // Send STOP condition
    i2c.cr1.modify(|_, w| w.stop().set_bit());
    Ok(())
}

pub fn who_am_i(i2c: &I2C1) -> Result<[u8; RX_BUFFER_SIZE], nb::Error<I2CError>> {
    let mut rx_buffer = [0];

    start(i2c);
    while write(i2c, LIS3DH_REG_WHOAMI as u8).is_err() {}
    // while write(i2c, 0 as u8).is_err {}
    // Read incoming bytes and ACK them
    while start(i2c).is_err() {}
    for i in 0..RX_BUFFER_SIZE {
        rx_buffer[i] = loop {
            if i == RX_BUFFER_SIZE - 1 {
                // Do not ACK the last byte received and send STOP
                if let Ok(byte) = read_nack(i2c) {
                    break byte;
                }
            } else {
                // ACK the byte after receiving
                if let Ok(byte) = read_ack(i2c) {
                    break byte;
                }
            }
        }
    }
    Ok(rx_buffer)
}