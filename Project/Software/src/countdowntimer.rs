pub struct CountdownTimer{
    startT: u32,
    isStarted: bool,
    buttonPressed: u32/*button*/,
}


impl CountdownTimer{
    pub fn newTimer() -> Self{
        CountdownTimer{
            startT: 0u32,
            isStarted: false,
            buttonPressed: 0u32,
        }
    }

    pub fn set_timer(&mut self){
        self.startT += 1;
    }

    pub fn pause_timer(&mut self){
        self.isStarted = false;                 //pauses the countdown
    }

    pub fn reset_timer(&mut self){
        self.startT = 0;
        self.isStarted = false;
    }

    pub fn get_button_pressed(&mut self) -> u32{
        return self.buttonPressed;
    }

    pub fn get_isStarted(&mut self) -> bool{
        return self.isStarted;
    }

    pub fn set_isStarted(&mut self, status:bool){
        self.isStarted = status;
    }

    pub fn get_startT(&mut self) -> u32{
        return self.startT;
    }

    pub fn set_startT(&mut self, value: u32){
        self.startT -= value;
    }
}

// #[interrupt(resources = [ITM, EXTI, LCD, COUNTDOWNTIMER], spawn = [start_timer])]
//     fn run_timer(){
//         if /*PC7 interrupt*/ && !(resources.COUNTDOWNTIMER.get_isStarted()){
//             resources.COUNTDOWNTIMER.set_isStarted(true);
//             spawn.start_timer().unwrap();
//         }
//         else if /*PC7  interrupt*/ && resources.COUNTDOWNTIMER.get_isStarted(){
//             resources.COUNTDOWNTIMER.pause_timer();
//         }
//         else if /*PC8 interrupt*/ && !(resources.COUNTDOWNTIMER.get_isStarted()){
//             resources.COUNTDOWNTIMER.set_timer();
//         }
//         else if /*PC9 interrupt*/{
//             resources.COUNTDOWNTIMER.reset_timer();
//         }
//     }

// pub fn run_timer(&mut self){
    //     if self.buttonPressed == /*PC7*/ && !(self.isStarted){    //pressed button 1
    //         self.isStarted = true;
    //         start_timer();
    //     }
    //     else if self.buttonPressed == /*PC7*/ && self.isStarted{
    //         pause_timer();
    //     }
    //     else if self.buttonPressed == /*PC8*/ && !(self.isStarted){
    //         set_time();
    //     }
    //     else if self.buttonPressed == /*PC9*/{
    //         reset_timer();
    //     }
    // }

    // fn start_timer(&mut self){
    //     if self.startT > 0{
    //         schedule.start_timer(Instant::now() + (64_000_000_u32).cycles()).unwrap();
    //         self.startT -= (1 as u32);
    //     }
    //     else{
    //         /*Possibly add function call to speaker*/
    //         self.isStarted = false;
    //     }  
    // }