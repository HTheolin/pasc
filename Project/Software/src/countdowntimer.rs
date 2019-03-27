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
