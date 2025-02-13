// use embedded_hal::blocking::spi::Write;
use hal::prelude::_embedded_hal_blocking_spi_Write as Write; 
use hal::prelude::_embedded_hal_digital_OutputPin as OutputPin;
use hal::stm32::{TIM5, SPI1};
use hal::timer::Timer;
use hal::time::U32Ext;
use stm32f4xx_hal::prelude::_embedded_hal_timer_CountDown as CountDown;
use hal::spi::{Spi, NoMiso};
use hal::gpio::{Alternate, AF5};
use hal::gpio::gpioa::{PA5, PA7};
use crate::pcd8544::Pcd8544;
use core::marker::{Unsize};

pub struct Pcd8544Spi<DC, CS> {
    dc: DC,
    cs: CS,
}

impl<DC, CS> Pcd8544Spi<DC, CS>
where
    DC: OutputPin,
    CS: OutputPin,
{
    pub fn new(dc: DC, cs: CS) -> Pcd8544Spi<DC, CS> {
        let mut pcd = Pcd8544Spi { dc, cs };
        return pcd;
    }
}

impl<DC, CS> Pcd8544 for Pcd8544Spi<DC, CS>
where
    DC: OutputPin,
    CS: OutputPin,
{
    fn command(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>, cmd: u8) {
        self.dc.set_low();
        self.cs.set_low();
        spi.write(&[cmd]);
        self.cs.set_high();
    }

    fn data(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>, data: &[u8]) {
        self.dc.set_high();
        self.cs.set_low();
        spi.write(data);
        self.cs.set_high();
    }
}
