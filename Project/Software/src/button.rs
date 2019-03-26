use core::any::{Any, TypeId};

use hal::stm32::{SYSCFG, EXTI, RCC};
use hal::stm32::{GPIOA, GPIOB, GPIOC};
use hal::gpio::{Edge};

macro_rules! impl_Button {
    ($GPIOX:ident, $PXX:ident, $BPXX:ident, $exticri: ident, $extix: ident, $moderx: ident, $pupdrx: ident, $mrx:ident, $trx:ident, $prx:ident, $idrx: ident) => {
        /// Button connected to pin Pxx
        pub const $BPXX: $PXX = $PXX;

        /// Pin Pxx. There's a button connected to this pin
        pub struct $PXX;

        impl $PXX {

            pub fn init(&self, gpiox: &$GPIOX, rcc: &RCC, syscfg: &SYSCFG, exti: &EXTI, edge: Edge, is_pullup: bool) {
                
                if gpiox.type_id() == TypeId::of::<GPIOA>() {
                   rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
                } else if gpiox.type_id() == TypeId::of::<GPIOB>() {
                   rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());
                } else if gpiox.type_id() == TypeId::of::<GPIOC>() {
                    rcc.ahb1enr.modify(|_, w| w.gpiocen().set_bit());
                }
                
                // Configure PC13 as input with pull-downs, RM0368 Table 23
                gpiox.moder.modify(|_, w| w.$moderx().bits(0) );
                if is_pullup {
                    gpiox.pupdr.modify(|_, w| unsafe { w.$pupdrx().bits(0b01) });
                } else {
                    gpiox.pupdr.modify(|_, w| unsafe { w.$pupdrx().bits(0b10) });
                }
                // System configuration controller clock enable
                rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());
                // Enable external interrupt RM0368 7.2.6
                if gpiox.type_id() == TypeId::of::<GPIOA>() {
                    syscfg
                        .$exticri
                        .modify(|_, w| unsafe { w.$extix().bits(0b0000) });
                } else if gpiox.type_id() == TypeId::of::<GPIOB>() {
                    syscfg
                        .$exticri
                        .modify(|_, w| unsafe { w.$extix().bits(0b0001) });
                } else if gpiox.type_id() == TypeId::of::<GPIOC>() {
                    syscfg
                        .$exticri
                        .modify(|_, w| unsafe { w.$extix().bits(0b0010) });
                }
                

                // Interrupt request from line 13 is not masked
                exti.imr.modify(|_, w| w.$mrx().set_bit());

                match edge {
                    Edge::RISING => {
                        exti.rtsr.modify(|_, w| w.$trx().set_bit());
                        exti.ftsr.modify(|_, w| w.$trx().clear_bit());
                    },
                    Edge::FALLING => {
                        exti.ftsr.modify(|_, w| w.$trx().set_bit());
                        exti.rtsr.modify(|_, w| w.$trx().clear_bit());
                    },
                    Edge::RISING_FALLING => {
                        exti.ftsr.modify(|_, w| w.$trx().set_bit());
                        exti.ftsr.modify(|_, w| w.$trx().set_bit());
                    }
                }

            }
            /// True if button is pressed, false otherwise.
            pub fn is_pressed(&self) -> bool {
                 unsafe { (*$GPIOX::ptr()).idr.read().$idrx().bit_is_set() }
            }

            /// Clear the pending external interrupt line used by the button, PR13
            pub fn clear_pending(&self, exti: &EXTI) {
                // RM0368 10.3.6 Pending register
                exti.pr.modify(|_, w| w.$prx().set_bit());
            }
        }
    }
}
 
impl_Button!(GPIOC, PC7, BPC7, exticr2, exti7, moder7, pupdr7, mr7, tr7, pr7, idr7);
impl_Button!(GPIOC, PC8, BPC8, exticr3, exti8, moder8, pupdr8, mr8, tr8, pr8, idr8);
impl_Button!(GPIOC, PC9, BPC9, exticr3, exti9, moder9, pupdr9, mr9, tr9, pr9, idr9);
impl_Button!(GPIOB, PB0, BPB0, exticr1, exti0, moder0, pupdr0, mr0, tr0, pr0, idr0);
impl_Button!(GPIOB, PB1, BPB1, exticr1, exti1, moder1, pupdr1, mr1, tr1, pr1, idr1);
impl_Button!(GPIOB, PB2, BPB2, exticr1, exti2, moder2, pupdr2, mr2, tr2, pr2, idr2);
impl_Button!(GPIOB, PB5, BPB5, exticr2, exti5, moder5, pupdr5, mr5, tr5, pr5, idr5);
impl_Button!(GPIOC, PC13, BPC13, exticr4, exti13, moder13, pupdr13, mr13, tr13, pr13, idr13);
