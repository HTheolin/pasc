pub enum Direction {
    X,
    Y,
    Z
}

pub struct Pedometer {
    threshold: f32,
    min_threshold: f32,
    max_value: f32,
    min_value:f32,
    steps: u32,
    direction: Direction,
}

impl Pedometer {
    pub fn new(min_threshold: f32) -> Self {
        Pedometer {
            threshold: 0f32,
            min_threshold: min_threshold,
            max_value: 0f32,
            min_value: 0f32,
            steps: 0u32,
            direction: Direction::Z,
        }
    }

    pub fn calc_threshold(&mut self) {
        self.threshold = (self.max_value + self.min_value) / 2.0;
    }

    pub fn calc_max(&mut self, x: f32, y: f32, z: f32) {
        let mut biggest = 0.0;
        if x > y {
            biggest = x;
        } else {
            biggest = y;
        }
        if biggest < z {
            biggest = z;
        }   
        self.direction = match biggest {
            x => Direction::X,
            y => Direction::Y,
            z => Direction::X,
        };
        if biggest > self.min_threshold {
            self.max_value = biggest;
        }
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }
    
    pub fn is_step(&mut self, step: f32) -> bool{
        if step > self.threshold {
            true
        } else {
            false
        }
    }

    pub fn add_step(&mut self) {
        self.steps += 1;
    }

    pub fn get_steps(&self) -> u32 {
        self.steps
    }

    pub fn reset_steps(&mut self) {
        self.steps = 0;
    }
}