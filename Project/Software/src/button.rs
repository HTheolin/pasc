use hal::stm32::{SYSCFG, EXTI};

use hal::gpio::{ExtiPin, Edge, Input, Floating, PullDown};
use hal::gpio::gpioc::{PC7, PC8, PC9};
use hal::gpio::gpiob::{PB0, PB1, PB2};
pub struct Button<T>(pub T);

macro_rules! impl_Button {
    ($GPIOX:ident, $PXX:ident) => {
        impl<'a> Button<$PXX<Input<Floating>>>
        {
            pub fn init(self, syscfg: &mut SYSCFG, exti: &mut EXTI, trigger: Edge) -> $PXX<Input<PullDown>> {
                
                let p_xx = self.0;
                let mut p_xx  = p_xx.into_pull_down_input();
                p_xx.make_interrupt_source(syscfg);
                p_xx.trigger_on_edge(exti, trigger);
                p_xx.enable_interrupt(exti);
                
                return p_xx;
            }
        }
    }
} 
impl_Button!(GPIOC, PC7);
impl_Button!(GPIOC, PC8);
impl_Button!(GPIOC, PC9);
impl_Button!(GPIOB, PB0);
impl_Button!(GPIOB, PB1);
impl_Button!(GPIOB, PB2);




// pub fn init(gpioc: GPIOC, pc7: PC7<Input<Floating>>, syscfg: &mut SYSCFG, exti: &mut EXTI){
                
//                 let mut pXx = gpioc.split().pc7;
//                 let mut pXx  = pXx.into_pull_down_input();

//                 pXx.make_interrupt_source(syscfg);
//                 pXx.trigger_on_edge(exti, Edge::FALLING);
//                 pXx.enable_interrupt(exti);
//             }