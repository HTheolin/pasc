pub const SAMPLELIMIT: usize = 50;

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
    x_samples: [f32; SAMPLELIMIT],
    y_samples: [f32; SAMPLELIMIT],
    z_samples: [f32; SAMPLELIMIT],
    sample_count: usize,
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
            x_samples: [0f32; SAMPLELIMIT],
            y_samples: [0f32; SAMPLELIMIT],
            z_samples: [0f32; SAMPLELIMIT],
            sample_count: 0usize,
        }
    }

    pub fn calc_threshold(&mut self) {
        self.threshold = (self.max_value + self.min_value) / 2.0;
    }

    pub fn calc_max(&mut self) {
        let mut max_x = 0.0;
        let mut max_y = 0.0;
        let mut max_z = 0.0;

        for value in self.x_samples.iter() {
            if *value > max_x {
                max_x = *value;
            }
        }
        for value in self.y_samples.iter() {
            if *value > max_y {
                max_y = *value;
            }
        }
        for value in self.z_samples.iter() {
            if *value > max_z {
                max_z = *value;
            }
        }

        let mut biggest = 0.0;
        if max_x > max_y {
            biggest = max_x;
        } else {
            biggest = max_y;
        }
        if biggest < max_z {
            biggest = max_z;
        }   
        self.direction = match biggest {
            max_x => Direction::X,
            max_y => Direction::Y,
            max_z => Direction::X,
        };
        if biggest > self.min_threshold {
            self.max_value = biggest;
        }
    }
    

    pub fn calc_min(&mut self) {
        let mut min = 10.0;
        
        match self.direction {
            Direction::X => for value in self.x_samples.iter() {
                                if *value < min && min != 0.0 {
                                    min = *value;
                                }
                            },
            Direction::Y => for value in self.y_samples.iter() {
                                if *value < min && min != 0.0 {
                                    min = *value;
                                }
                            }
            Direction::Z => for value in self.z_samples.iter() {
                                if *value < min && min != 0.0 {
                                    min = *value;
                                }
                            }
        }
        self.min_value = min;
    }
    
    pub fn get_max(&self) -> f32 {
        self.max_value
    }

    pub fn get_min(&self) -> f32 {
        self.min_value
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


    /// Stores samples read in 3 [f32] with size of SAMPLELIMIT
    pub fn set_samples(&mut self, x_g: f32, y_g: f32, z_g: f32) {
        if self.sample_count < SAMPLELIMIT {
            self.x_samples[self.sample_count] = x_g;
            self.y_samples[self.sample_count] = y_g;
            self.z_samples[self.sample_count] = z_g;
        }
    }


    pub fn increment_sample(&mut self) {
        self.sample_count = self.sample_count.saturating_add(1);
    }

    pub fn get_samples(&self) -> usize {
        self.sample_count
    }

    pub fn reset_samples(&mut self) {
        self.x_samples.iter_mut().for_each(|x| *x = 0.0);
        self.y_samples.iter_mut().for_each(|y| *y = 0.0);
        self.z_samples.iter_mut().for_each(|z| *z = 0.0);
        self.sample_count = 0;
    }

    pub fn get_xsamples(&self) -> &[f32] {
        &self.x_samples
    }

    pub fn get_ysamples(&self) -> &[f32] {
        &self.y_samples
    }

    pub fn get_zsamples(&self) -> &[f32] {
        &self.z_samples
    }

    pub fn get_threshold(&self) -> f32 {
        self.threshold
    }
}
