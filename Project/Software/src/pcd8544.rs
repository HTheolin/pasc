use crate::pcd8544_spi;
use crate::font;
use hal::spi::{Spi, PinSck, NoMiso, PinMosi};
use hal::stm32::SPI1;
use hal::gpio::{Alternate, AF5};
use hal::gpio::gpioa::{PA5, PA7};
pub use pcd8544_spi::Pcd8544Spi;

const PCD8544_X_PIXELS: usize = 84;
const PCD8544_Y_PIXELS: usize = 48;

pub trait Pcd8544 {
    fn command(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>, cmd: u8);
    fn data(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>, data: &[u8]);

    fn init(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>) {
        self.command(spi, 0x21); // chip active; horizontal addressing mode (V = 0); use extended instruction set (H = 1)
        // set LCD Vop (contrast), which may require some tweaking:
        self.command(spi, 0xB4); // 0x80 to 0xFF. 0xA0 works for Henrik PCB, 0xBF is good for Simon PCB.
        self.command(spi, 0x04); // set temp coefficient
        self.command(spi, 0x13); // LCD bias mode 1:48: try 0x13 or 0x14

        self.command(spi, 0x20); // we must send 0x20 before modifying the display control mode
        self.command(spi, 0x0C); // set display control to normal mode: 0x0D for inverse

        self.clear(spi);
    }

    fn print_char(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>, c: u8) {
        let i = (c as usize) - 0x20;

        self.data(spi, &font::ASCII[i]);
        self.data(spi, &[0x00]);
    }

    fn print(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>, s: &str) {
        for c in s.bytes() {
            self.print_char(spi, c);
        }
    }

    fn print_bytes(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>, s: &[u8]) {
        for c in s {
            self.print_char(spi, *c);
        }
    }

    fn set_position(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>, x: u8, y: u8) {
        assert!(x <= 84);
        assert!(y < 6);

        self.command(spi, 0x40 + y);
        self.command(spi, 0x80 + x);
    }

    // note: data direction is vertical: [1 2 3 4 5 6]
    // 1 3 5
    // 2 4 6
    fn draw_buffer(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>, buffer: &[u8]) {
        self.command(spi, 0x22); // vertical addressing
        self.set_position(spi, 0, 0);
        self.data(spi, buffer);
        self.command(spi, 0x20); // horizontal addressing
        self.set_position(spi, 0, 0);
    }

    fn draw_buffer_horizontal(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>, buffer: &[u8]) {
        self.command(spi, 0x20); // horizontal addressing
        self.set_position(spi, 0, 0);
        self.data(spi, buffer);
        self.command(spi, 0x20); // horizontal addressing
        self.set_position(spi, 0, 0);
    }
     
    fn set_pixel(&mut self, x: usize, y: usize, value: bool, buffer: &mut [u8]) {
         if x >= PCD8544_X_PIXELS || y >= PCD8544_Y_PIXELS {
             return;
         }
        let bank = y / 8;
        let bit_mask = 1 << (y % 8);
        let byte = &mut buffer[(PCD8544_X_PIXELS * bank as usize) + x as usize];
        if value {
            *byte |= bit_mask;
        } else {
            *byte &= !bit_mask;
        }
    }

    fn clear(&mut self, spi: &mut Spi<SPI1, (PA5<Alternate<AF5>>, NoMiso, PA7<Alternate<AF5>>)>) {
        self.set_position(spi, 0, 0);
        self.command(spi, 0x20); // we must send 0x20 before modifying the display control mode
        self.command(spi, 0x00); // set display control to blank mode
        self.command(spi, 0x20); // we must send 0x20 before modifying the display control mode
        self.command(spi, 0x0C); // set display control to normal mode: 0x0D for inverse
        self.set_position(spi, 0, 0);
    }
}
