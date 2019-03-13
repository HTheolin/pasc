use hal::stm32::{SYSCFG, EXTI, GPIOC, GPIOB};

use hal::gpio::{ExtiPin, Edge, GpioExt, Input, Floating, PullDown};
use hal::gpio::gpioc::{PC7, PC8, PC9};
use hal::gpio::gpiob::{PB0, PB1, PB2};
pub struct Button<T>(pub T);

macro_rules! impl_Button {
    ($GPIOX:ident, $PXX:ident, $pxx:ident) => {
        impl<'a> Button<$PXX<Input<Floating>>>
        {
            pub fn init(self, syscfg: &mut SYSCFG, exti: &mut EXTI) -> $PXX<Input<PullDown>> {
                
                let p_xx = self.0;
                let mut p_xx  = p_xx.into_pull_down_input();
                
                p_xx.make_interrupt_source(syscfg);
                p_xx.trigger_on_edge(exti, Edge::FALLING);
                p_xx.enable_interrupt(exti);
                return p_xx;
            }
            pub fn clear_interrupt(mut self, exti: &mut EXTI){
                self.0.clear_interrupt_pending_bit(exti)
            }
        }
    }
} 
impl_Button!(GPIOC, PC7, pc7);
impl_Button!(GPIOC, PC8, pc8);
impl_Button!(GPIOC, PC9, pc9);
impl_Button!(GPIOB, PB0, pb0);
impl_Button!(GPIOB, PB1, pb1);
impl_Button!(GPIOB, PB2, pb2);




// pub fn init(gpioc: GPIOC, pc7: PC7<Input<Floating>>, syscfg: &mut SYSCFG, exti: &mut EXTI){
                
//                 let mut pXx = gpioc.split().pc7;
//                 let mut pXx  = pXx.into_pull_down_input();

//                 pXx.make_interrupt_source(syscfg);
//                 pXx.trigger_on_edge(exti, Edge::FALLING);
//                 pXx.enable_interrupt(exti);
//             }