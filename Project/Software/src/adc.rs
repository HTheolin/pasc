extern crate stm32f4xx_hal as hal;
use hal::stm32::{ADC1, DMA2, RCC};

/// Input channel associated to ADC1
#[derive(Clone, Copy, Debug)]
pub enum AdcChannel {
    /// ADC1_IN0 = PA0
    _0 = 0,
    /// ADC1_IN1 = PA1
    _1 = 1,
}

pub struct Adc<'a>(pub &'a ADC1);

impl<'a> Adc<'a> {
    /// Enables the ADC input
    pub fn enable_input(
        &self,
        input: AdcChannel,
        sq: u8,
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
                .modify(|_, w| unsafe { w.l().bits(sq.wrapping_sub(1)) });
        }
    }

    pub fn init(&self, dma2: &DMA2, rcc: &RCC) {
        let adc1 = self.0;
        // enable ADC1, DMA1, GPIOA, TIM2
        // rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
        // rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());
        // rcc.ahb1enr.modify(|_, w| w.gpiocen().set_bit());
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

    /// Enables the ADC
    pub fn enable(&self) {
        self.0.cr2.modify(|_, w| w.adon().set_bit());
    }

}