use libm::F32Ext;

pub const SAMPLELIMIT: usize = 40;
pub const STEPWINDOW: u32 = 200;

#[derive(Debug)]
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
    vec_samples: [f32; SAMPLELIMIT],
    sample_count: usize,
}

/// Pedometer that uses a dynamic threshold algorith to detect steps. 
/// Units are m/s². 
/// The period between new threshold updates are set by SAMPLELIMIT
impl Pedometer {
    /// Creates a new instance of a pedometer, set start threshold and min threshold in m/s²
    pub fn new(threshold: f32, min_threshold: f32) -> Self {
        Pedometer {
            threshold: threshold,
            min_threshold: min_threshold,
            max_value: 0f32,
            min_value: 0f32,
            steps: 0u32,
            vec_samples: [0f32; SAMPLELIMIT],
            sample_count: 0usize,
        }
    }

    pub fn calc_threshold(&mut self) {
        self.threshold = (self.max_value + self.min_value) / 2.0;
    }

    /// Calculates a new max value from samples gathered.
    /// Sets the axis with highest value as direction downwards
    pub fn calc_max(&mut self) {
        let mut max_vec = 0.0;
    
        for value in self.vec_samples.iter() {
            if *value > max_vec {
                max_vec = *value;
            }
        }

        self.max_value = max_vec;
    }

    /// Calculates new min value form the samples gathered.
    pub fn calc_min(&mut self) {
        let mut min = 100.0;

        for value in self.vec_samples.iter() {
            if *value < min && *value != 0.0 {
                min = *value;
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
    
    pub fn vector_down(&mut self, x: f32, y: f32, z: f32) -> f32 {
        let vec = (x*x + y*y + z*z).sqrt();
        vec
    }

    pub fn is_step(&mut self, step: f32) -> bool{
        if step > self.threshold && self.max_value - self.min_value > self.min_threshold {
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
    pub fn add_sample(&mut self, vec_g: f32) {
        if self.sample_count < SAMPLELIMIT {
            self.vec_samples[self.sample_count] = vec_g;
        }
    }

    pub fn increment_sample(&mut self) {
        self.sample_count = self.sample_count.saturating_add(1);
    }

    pub fn get_samples(&self) -> usize {
        self.sample_count
    }

    pub fn reset_samples(&mut self) {
        self.vec_samples.iter_mut().for_each(|x| *x = 0.0);
        self.sample_count = 0;
    }

    pub fn get_vec_samples(&self) -> &[f32] {
        &self.vec_samples
    }

    pub fn get_threshold(&self) -> f32 {
        self.threshold
    }
}
