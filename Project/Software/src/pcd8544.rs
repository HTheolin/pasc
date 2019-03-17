use crate::pcd8544_spi;
use crate::font;
use hal::spi::{Spi, Pins};
use hal::stm32::SPI1;
pub use pcd8544_spi::Pcd8544Spi;

pub trait Pcd8544 {
    fn command(&mut self, spi: &mut Spi<SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5>>)>, cmd: u8);
    fn data(&mut self, spi: &mut Spi<SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5>>)>, data: &[u8]);

    fn init(&mut self, spi: &mut Spi<SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5>>)>) {
        self.command(spi, 0x21); // chip active; horizontal addressing mode (V = 0); use extended instruction set (H = 1)
                            // set LCD Vop (contrast), which may require some tweaking:
        self.command(spi, 0xB8); // try 0xB1 (for 3.3V red SparkFun), 0xB8 (for 3.3V blue SparkFun), 0xBF if your display is too dark, or 0x80 to 0xFF if experimenting
        self.command(spi, 0x04); // set temp coefficient
        self.command(spi, 0x14); // LCD bias mode 1:48: try 0x13 or 0x14

        self.command(spi, 0x20); // we must send 0x20 before modifying the display control mode
        self.command(spi, 0x0C); // set display control to normal mode: 0x0D for inverse

        self.clear(spi);
    }

    fn print_char(&mut self, spi: &mut Spi<SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5>>)>, c: u8) {
        let i = (c as usize) - 0x20;

        self.data(spi, &font::ASCII[i]);
        self.data(spi, &[0x00]);
    }

    fn print(&mut self, spi: &mut Spi<SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5>>)>, s: &str) {
        for c in s.bytes() {
            self.print_char(spi, c);
        }
    }

    fn set_position(&mut self, spi: &mut Spi<SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5>>)>, x: u8, y: u8) {
        assert!(x <= 84);
        assert!(y < 6);

        self.command(spi, 0x40 + y);
        self.command(spi, 0x80 + x);
    }

    // note: data direction is vertical: [1 2 3 4 5 6]
    // 1 3 5
    // 2 4 6
    fn draw_buffer(&mut self, spi: &mut Spi<SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5>>)>, buffer: &[u8; 6*84]) {
        self.command(spi, 0x22); // vertical addressing
        self.set_position(spi, 0, 0);
        self.data(spi, buffer);
        self.command(spi, 0x20); // horizontal addressing
        self.set_position(spi, 0, 0);
    }

    fn clear(&mut self, spi: &mut Spi<SPI1, (hal::gpio::gpioa::PA5<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpioa::PA7<hal::gpio::Alternate<hal::gpio::AF5>>)>) {
        self.set_position(spi, 0, 0);
        self.data(spi, &[0u8; 6*84]);
        self.set_position(spi, 0, 0);
    }
}
