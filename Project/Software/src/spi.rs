//! Serial Peripheral Interface
//!
//! You can use the `Spi` interface with these SPI instances
//!
//! # SPI1
//!
//! - NSS = PA4
//! - SCK = PA5
//! - MISO = PA6
//! - MOSI = PA7
//!
//! # SPI2
//!
//! - NSS = PB12
//! - SCK = PB13
//! - MISO = PB14
//! - MOSI = PB15
//!
//! # SPI3
//!
//! - NSS = PA15
//! - SCK = PB3
//! - MISO = PB4
//! - MOSI = PB5
//!
use core::any::{Any, TypeId};
use core::ptr;

use nb;
use hal::stm32::{SPI1, SPI2, SPI3, GPIOA, GPIOB, RCC};
use hal::spi;
/// SPI result
pub type Result<T> = ::core::result::Result<T, nb::Error<Error>>;

/// SPI error
#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
    #[doc(hidden)] _Extensible,
}

/// Serial Peripheral Interface
pub struct Spi<'a, T>(pub &'a T)
where
    T: 'a;

/// Serial Peripheral Interface
macro_rules! impl_Spi {
    ($S:ident) => {
        impl<'a> Spi<'a, $S>
        {
            /// Initializes the SPI
            pub fn init(&self,
                    gpioa: &GPIOA, // TODO: Make these optional/implement custom init for each SPI
                    gpiob: &GPIOB,
                    rcc: &RCC) {
                let spi = self.0;

                if spi.type_id() == TypeId::of::<SPI1>() {
                    // enable SPI1, GPIOA
                    rcc.apb2enr.modify(|_, w| {
                        w.spi1en().set_bit()
                    });
                    rcc.ahb1enr.modify(|_, w| {
                        w.gpioaen().set_bit()
                    });

                    // NSS = PA4 = Alternate function push pull
                    // SCK = PA5 = Alternate function push pull
                    // MISO = PA6 = Alternate function open drain
                    // MOSI = PA7 = Alternate function push pull

                    // DM00102166 - Alternate function AF5, Table 9
                    gpioa.afrl.modify(|_, w|
                        w.afrl4().bits(5)
                        .afrl5().bits(5)
                        .afrl6().bits(5)
                        .afrl7().bits(5));
                    // RM0368 8.3 Table 23
                    // Highest output speed
                    gpioa.ospeedr.modify(|_, w|
                        w.ospeedr4().bits(0b11)
                        .ospeedr5().bits(0b11)
                        .ospeedr6().bits(0b11)
                        .ospeedr7().bits(0b11));
                    // Alternate function mode
                    gpioa.moder.modify(|_, w|
                        w.moder4().bits(2)
                        .moder5().bits(2)
                        .moder6().bits(2)
                        .moder7().bits(2));
                    // Push pull, MISO open drain
                    gpioa.otyper.modify(|_, w|
                        w.ot4().clear_bit()
                        .ot5().clear_bit()
                        .ot6().set_bit()
                        .ot7().clear_bit()
                    );
                    // No pull up/down except MISO
                    gpioa.pupdr.modify(|_, w| unsafe {
                        w.pupdr4().bits(0)
                        .pupdr5().bits(0)
                        .pupdr6().bits(1)
                        .pupdr7().bits(0)
                    });

                } else if spi.type_id() == TypeId::of::<SPI2>() {
                    // enable SPI2, GPIOB
                    rcc.apb1enr.modify(|_, w| {
                        w.spi2en().set_bit()
                    });
                    rcc.ahb1enr.modify(|_, w| {
                        w.gpioben().set_bit()
                    });

                    // NSS = PB12 = Alternate function push pull
                    // SCK = PB13 = Alternate function push pull
                    // MISO = PB14 = Alternate function open drain
                    // MOSI = PB15 = Alternate function push pull

                    // DM00102166 - Alternate function AF5, Table 9
                    gpiob.afrh.modify(|_, w| unsafe {
                        w.afrh12().bits(5)
                        .afrh13().bits(5)
                        .afrh14().bits(5)
                        .afrh15().bits(5)
                    });
                    // RM0368 8.3 Table 23
                    // Highest output speed
                    gpiob.ospeedr.modify(|_, w| unsafe {
                        w.ospeedr12().bits(0b11)
                        .ospeedr13().bits(0b11)
                        .ospeedr14().bits(0b11)
                        .ospeedr15().bits(0b11)
                    });
                    // Alternate function mode
                    gpiob.moder.modify(|_, w| unsafe {
                        w.moder12().bits(2)
                        .moder13().bits(2)
                        .moder14().bits(2)
                        .moder15().bits(2)
                    });
                    // Push pull, MISO open drain
                    gpiob.otyper.modify(|_, w|
                        w.ot12().clear_bit()
                        .ot13().clear_bit()
                        .ot14().set_bit()
                        .ot15().clear_bit()
                    );
                    // No pull up/down except MISO
                    gpiob.pupdr.modify(|_, w| unsafe {
                        w.pupdr12().bits(0)
                        .pupdr13().bits(0)
                        .pupdr14().bits(1)
                        .pupdr15().bits(0)
                    });

                } else if spi.type_id() == TypeId::of::<SPI3>() {
                    // enable SPI3, GPIOA/B
                    rcc.apb1enr.modify(|_, w| {
                        w.spi3en().set_bit()
                    });
                    rcc.ahb1enr.modify(|_, w| {
                        w.gpioaen().set_bit()
                    });
                    rcc.ahb1enr.modify(|_, w| {
                        w.gpioben().set_bit()
                    });

                    // NSS = PA15 = Alternate function push pull
                    // SCK = PB3 = Alternate function push pull
                    // MISO = PB4 = Alternate function open drain
                    // MOSI = PB5 = Alternate function push pull

                    // DM00102166 - Alternate function AF6, Table 9
                    gpioa.afrh.modify(|_, w| w.afrh15().bits(5));
                    gpiob.afrl.modify(|_, w| unsafe {
                        w.afrl3().bits(6)
                        .afrl4().bits(6)
                        .afrl5().bits(6)
                    });
                    // RM0368 8.3 Table 23
                    // Highest output speed
                    gpioa.ospeedr.modify(|_, w| w.ospeedr15().bits(0b11));
                    gpiob.ospeedr.modify(|_, w| unsafe {
                        w.ospeedr3().bits(0b11)
                        .ospeedr4().bits(0b11)
                        .ospeedr5().bits(0b11)
                    });
                    // Alternate function mode
                    gpioa.moder.modify(|_, w|  w.moder15().bits(2));
                    gpiob.moder.modify(|_, w| unsafe {
                        w.moder3().bits(2)
                        .moder4().bits(2)
                        .moder5().bits(2)
                    });
                    // Push pull, MISO open drain
                    gpioa.otyper.modify(|_, w| w.ot15().clear_bit());
                    gpiob.otyper.modify(|_, w|
                        w.ot3().clear_bit()
                        .ot4().set_bit()
                        .ot5().clear_bit()
                    );
                    // No pull up/down except MISO
                    gpioa.pupdr.modify(|_, w|unsafe {
                        w.pupdr15().bits(0)});
                    gpiob.pupdr.modify(|_, w|unsafe {
                        w
                        .pupdr3().bits(0)
                        .pupdr4().bits(1)
                        .pupdr5().bits(0)
                    });
                }

                // enable SS output
                spi.cr2.write(|w| w.ssoe().set_bit());

                // RM0368 20.5.1 SPI control register 1
                spi.cr1.write(|w| unsafe {
                    w.br().bits(0b100) // CLK / 32 prescaler
                        .ssi().set_bit() // NSS
                        .ssm().set_bit() // Software chip select
                        .mstr().set_bit() // Master SPI mode
                        .cpol().set_bit() // CK to 1 when idle
                        .lsbfirst().clear_bit() // MSB first
                        .dff().clear_bit() // 8 bit frames
                        .bidimode().clear_bit() // 2-line unidirectional mode
                });
            }

            /// Disables the SPI bus
            ///
            /// **NOTE** This drives the NSS pin high
            pub fn disable(&self) {
                self.0.cr1.modify(|_, w| w.spe().clear_bit())
            }

            /// Enables the SPI bus
            ///
            /// **NOTE** This drives the NSS pin low
            pub fn enable(&self) {
                self.0.cr1.modify(|_, w| w.spe().set_bit())
            }
        }
    }
}

impl_Spi!(SPI1);
impl_Spi!(SPI2);
impl_Spi!(SPI3);
