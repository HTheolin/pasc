use libm::F32Ext;

pub const SAMPLELIMIT: usize = 40;
pub const STEPWINDOW: u32 = 200;
pub const Y_Offset: f32 = 240.0;
pub const SCALE: f32 = - 12.232415902;
pub struct Pedometer {
    threshold: f32,
    min_threshold: f32,
    max_value: f32,
    min_value:f32,
    steps: u32,
    vec_samples: [f32; SAMPLELIMIT],
    sample_count: usize,
    last_values: [f32; 6],
    last_directions: [f32; 6],
    last_extremes: [[f32;6];6],
    last_diff: [f32;6],
    last_match: i8,
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
            last_values: [0f32; 6],
            last_directions: [0f32; 6],
            last_extremes: [[0f32;6];6],
            last_diff: [0f32;6],
            last_match: -1,
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

    pub fn is_step(&mut self, step: f32) -> bool {
        if step > self.threshold && self.max_value - self.min_value > self.min_threshold {
            true
        } else {
            false
        }
    }

    pub fn detect_step(&mut self, values: [f32; 3]) -> bool {
        let mut is_step = false;
        let mut v_sum = 0.0;

        for axis in values.iter() {
            let v = Y_Offset * axis * SCALE;
            v_sum += v;
        }

        let k: usize = 0;
        let v = v_sum / 3.0;

        let mut direction = 0.0;
        if v > self.last_values[k] {
            direction = 1.0;
        } else if v < self.last_values[k] {
            direction = -1.0;
        }

        if direction == - self.last_directions[k] {
            let mut ext_type = 1;
            if direction > 0.0 {
                ext_type = 0;
            }
            self.last_extremes[ext_type][k] = self.last_values[k];
            let mut  diff = self.last_extremes[ext_type][k] - self.last_extremes[1 - ext_type][k];
            if diff < 0.0 {
                diff = diff * -1.0;
            }

            if diff > self.threshold {
            
                let is_almost_as_large_as_previous = diff > (self.last_diff[k]*2.0/3.0);
                let is_previous_large_enough = self.last_diff[k] > (diff/3.0);
                let is_not_contra = self.last_match != 1 - (ext_type as i8);

                if is_almost_as_large_as_previous && is_previous_large_enough && is_not_contra {            
                    self.last_match = ext_type as i8;
                    is_step = true;
                } else {
                    self.last_match = -1;
                }
            }
            self.last_diff[k] = diff;
        }
        self.last_directions[k] = direction;
        self.last_values[k] = v;   
        is_step
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
