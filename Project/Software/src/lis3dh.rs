extern crate embedded_hal;

use nb;
use hal::stm32::{I2C1, GPIOB, RCC};
use cortex_m::{asm};

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


pub enum Range
{
  LIS3DH_RANGE_16_G         = 0b11,   // +/- 16g
  LIS3DH_RANGE_8_G           = 0b10,   // +/- 8g
  LIS3DH_RANGE_4_G           = 0b01,   // +/- 4g
  LIS3DH_RANGE_2_G           = 0b00    // +/- 2g (default value)
}

pub enum Divider
{
    DIV_16_G = 1365,
    DIV_8_G = 4096,
    DIV_4_G = 8190,
    DIV_2_G = 16380
}

pub enum Axis
{
  LIS3DH_AXIS_X         = 0x0,
  LIS3DH_AXIS_Y         = 0x1,
  LIS3DH_AXIS_Z         = 0x2,
}


/* Used with register 0x2A (LIS3DH_REG_CTRL_REG1) to set bandwidth */
pub enum Datarate
{
  LIS3DH_DATARATE_400_HZ     = 0b0111, //  400Hz 
  LIS3DH_DATARATE_200_HZ     = 0b0110, //  200Hz
  LIS3DH_DATARATE_100_HZ     = 0b0101, //  100Hz
  LIS3DH_DATARATE_50_HZ      = 0b0100, //   50Hz
  LIS3DH_DATARATE_25_HZ      = 0b0011, //   25Hz
  LIS3DH_DATARATE_10_HZ      = 0b0010, // 10 Hz
  LIS3DH_DATARATE_1_HZ       = 0b0001, // 1 Hz
  LIS3DH_DATARATE_POWERDOWN  = 0,
  LIS3DH_DATARATE_LOWPOWER_1K6HZ  = 0b1000,
  LIS3DH_DATARATE_LOWPOWER_5KHZ  =  0b1001,

}

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
         
        }
        // If we got NACK and tx empty, use ACK pulling:
        i2c.sr1.modify(|_,w| w.af().clear_bit());
    }
    i2c.cr1.modify(|_,w|  w.ack().set_bit());
    // Send START condition
    i2c.cr1.modify(|_, w| w.start().set_bit());
    // Wait for repeated start generation
    while i2c.sr1.read().sb().bit_is_clear() {}

    while {
        let sr2 = i2c.sr2.read();
        sr2.msl().bit_is_clear() && sr2.busy().bit_is_clear()
    } {}

    Ok(())
}

pub fn write(i2c: &I2C1, bytes: &mut [u8]) -> Result<(), nb::Error<I2CError>> {

    i2c.dr.write(|w| unsafe { w.bits(u32::from(ADDRESS) << 1) });
    
    // Wait for end of address transmission
    while i2c.sr1.read().addr().bit_is_clear() {
        if i2c.sr1.read().af().bit_is_set() {
            return Err(nb::Error::Other(I2CError::Timeout));
        }
    }
    i2c.sr2.read();

    // Send bytes
    for c in bytes {
        send_byte(i2c, *c)?;
    }
    Ok(())
}

/// Read a byte and respond with ACK
pub fn read_ack(i2c: &I2C1, buffer: &mut [u8]) -> Result<(), nb::Error<I2CError>> {

    i2c.dr.write(|w| unsafe { w.bits((u32::from(ADDRESS) << 1) + 1) });
    
    // Wait for end of address transmission
    while i2c.sr1.read().addr().bit_is_clear() {
        // if i2c.sr1.read().af().bit_is_set() {
        //     return Err(nb::Error::Other(I2CError::Timeout));
        // }
        let _sr2 = i2c.sr2.read().bits();
    }

    i2c.sr2.read();

    let mut i = 0;
    let size = buffer.len();

    for c in buffer {
        if i == size-1 {
            i2c.cr1.modify(|_,w|  w.ack().clear_bit());
        }
        *c = recv_byte(i2c)?;
        i += 1;
    }

    i2c.cr1.modify(|_, w| w.stop().set_bit());
    Ok(())
}

fn recv_byte(i2c: &I2C1) -> Result<u8, nb::Error<I2CError>> {
        while i2c.sr1.read().rx_ne().bit_is_clear() {}
        let value = i2c.dr.read().bits() as u8;
        Ok(value)
}

fn send_byte(i2c: &I2C1, byte: u8) -> Result<(), nb::Error<I2CError>> {
    while i2c.sr1.read().tx_e().bit_is_clear() {}

    i2c.dr.write(|w| unsafe { w.bits(u32::from(byte)) });

    while {
            let sr1 = i2c.sr1.read();

            // If we received a NACK, then this is an error
            if sr1.af().bit_is_set() {
                nb::Error::Other(I2CError::NACK);
            }

            sr1.btf().bit_is_clear()
    } {}

    Ok(())
}

/// High res & BDU enabled, DRDY on INT1 and 400Hz datarate
pub fn setup(i2c: &I2C1) {
    //High res & BDU
    write_register(i2c, &mut [LIS3DH_REG_CTRL1, 0x07]).unwrap();

    write_register(i2c, &mut [LIS3DH_REG_CTRL4, 0x88]).unwrap();
    //DRDY on INT1
    write_register(i2c, &mut [LIS3DH_REG_CTRL3, 0x10]).unwrap();
    //Interrupt active high = bit 2 set low bit 2 clear 
    write_register(i2c, &mut [LIS3DH_REG_CTRL6, 0x00]).unwrap();
}

/// Set datarate for the accelerometer update
pub fn set_datarate(i2c: &I2C1, datarate: Datarate) {
    let mut ctl1 = [0; 1];
    write_read_register(i2c, LIS3DH_REG_CTRL1, &mut ctl1).unwrap();
    ctl1[0] &= !(0xF0);
    ctl1[0] |= (datarate as u8) << 4;
    write_register(i2c, &mut [LIS3DH_REG_CTRL1, ctl1[0]]).unwrap();
}

/// Sets the range of the accelerometer accurary
pub fn set_range(i2c: &I2C1, range: Range) {
    let mut r = [0; 1];
    write_read_register(i2c, LIS3DH_REG_CTRL4, &mut r).unwrap();
    r[0] &= !(0x30);
    r[0] |= (range as u8) << 4;
    write_register(i2c, &mut [LIS3DH_REG_CTRL4, r[0]]).unwrap();
}

/// Enable interrupt for clicks, 
/// c nr of taps, 0 disable, 1 single, 2 double
/// clickthreash, threashold for the interrupt to be set
///     this strongly depend on the range! for 16G, try 5-10
///     for 8G, try 10-20. for 4G try 20-40. for 2G try 40-80
///
/// timelimit 
/// timelatency latency for interrupt after click was registered
/// timewindow How often a click interrupt is allowed
/// 
pub fn set_click_interrupt(i2c: &I2C1, c: u8, clickthresh: u8, timelimit: u8, timelatency: u8, timewindow: u8) {
    if c == 0 {
        //disable int
        let mut r = [0; 1];
        write_read_register(i2c, LIS3DH_REG_CTRL3, &mut r).unwrap();
        r[0] &= !(0x80); // turn off I1_CLICK
        write_register(i2c, &mut [LIS3DH_REG_CTRL3, r[0]]).unwrap();
        write_register(i2c, &mut [LIS3DH_REG_CLICKCFG, 0]).unwrap();
        return;
    }
    // else...

    write_register(i2c, &mut [LIS3DH_REG_CTRL3, 0x80]).unwrap(); // turn on int1 click
    write_register(i2c, &mut [LIS3DH_REG_CTRL5, 0x08]).unwrap(); // latch interrupt on int1

    if c == 1 {
        write_register(i2c, &mut [LIS3DH_REG_CLICKCFG, 0x15]).unwrap(); // turn on all axes & singletap
    } else if c ==2 {
        write_register(i2c, &mut [LIS3DH_REG_CLICKCFG, 0x2A]).unwrap(); // turn on all axes & doubletap
    }
    write_register(i2c, &mut [LIS3DH_REG_CLICKTHS, clickthresh]).unwrap(); // arbitrary
    write_register(i2c, &mut [LIS3DH_REG_TIMELIMIT, timelimit]).unwrap(); // arbitrary
    write_register(i2c, &mut [LIS3DH_REG_TIMELATENCY, timelatency]).unwrap(); // arbitrary
    write_register(i2c, &mut [LIS3DH_REG_TIMEWINDOW, timewindow]).unwrap(); // arbitrary
}


/// Send STOP condition
pub fn stop(i2c: &I2C1) -> Result<(), nb::Error<I2CError>> {
    // Disable ACK
    i2c.cr1.modify(|_,w|  w.ack().clear_bit());

    let _sr2 = i2c.sr2.read();
    
    // if i2c.sr1.read().tx_e().bit_is_clear() && i2c.sr1.read().btf().bit_is_clear() {
    //     return Err(nb::Error::WouldBlock);
    // }
    // Send STOP condition
    i2c.cr1.modify(|_, w| w.stop().set_bit());
    Ok(())
}

pub fn who_am_i(i2c: &I2C1, buffer: &mut [u8]) -> Result<(), nb::Error<I2CError>> {
    enable(i2c);
    while start(i2c).is_err() {};
    let mut rx_buffer = [0; 2];
    while write(i2c, &mut [LIS3DH_REG_WHOAMI as u8]).is_err() {}
    while start(i2c).is_err() {};
    read_ack(i2c, buffer);
    Ok(())
}

pub fn write_read_register(i2c: &I2C1, reg: u8, buffer: &mut [u8]) -> Result<(), nb::Error<I2CError>> {
    enable(i2c);
    while start(i2c).is_err() {};
    while write(i2c, &mut [reg as u8]).is_err() {}
    while start(i2c).is_err() {};
    read_ack(i2c, buffer);
    Ok(())
}

pub fn write_register(i2c: &I2C1, data: &mut [u8]) -> Result<(), nb::Error<I2CError>> {
    enable(i2c);
    while start(i2c).is_err() {};
    while write(i2c, data).is_err() {};
    while stop(i2c).is_err() {};
    Ok(())
}


pub fn read_accelerometer(i2c: &I2C1, data: &mut [u8]) -> Result<(), nb::Error<I2CError>> {
    // I2Cinterface->write(LIS3DH_REG_OUT_X_L | 0x80);
    enable(i2c);
    while start(i2c).is_err() {};
    while write(i2c, &mut [LIS3DH_REG_OUT_X_L | 0x80]).is_err() {};
    while start(i2c).is_err() {};
    read_ack(i2c, data);
    Ok(())
}