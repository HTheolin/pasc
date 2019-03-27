use crate::filter::Filter;

const ACCEL_RING_SIZE: usize = 50;
const VEL_RING_SIZE: usize = 10;
const THRESHOLD_SIZE: usize = 50;
const THRESHOLD_DIVITER: f32 = 20.0;

pub struct Step {
    step_threshold: f32,
    min_threshold: f32,
    accel_ring_counter: usize,
    accel_ring_x: [f32; ACCEL_RING_SIZE],
    accel_ring_y: [f32; ACCEL_RING_SIZE],
    accel_ring_z: [f32; ACCEL_RING_SIZE],
    vel_estimate_samples: [f32; ACCEL_RING_SIZE],
    vel_ring_counter: usize,
    vel_ring: [f32; VEL_RING_SIZE],
    steps: u32,
    last_directions: [f32; 6],
    last_extremes: [[f32;6];6],
    last_diff: f32,
    last_velocity: f32,
    last_match: i8,
}

impl Step {
    pub fn new(threshold: f32, min_threshold: f32) -> Self {
        Step {
            step_threshold: threshold,
            min_threshold,
            accel_ring_counter: 0usize,
            accel_ring_x: [0f32; ACCEL_RING_SIZE],
            accel_ring_y: [0f32; ACCEL_RING_SIZE],
            accel_ring_z: [0f32; ACCEL_RING_SIZE],
            vel_estimate_samples: [0f32; ACCEL_RING_SIZE],
            vel_ring_counter: 0usize,
            vel_ring: [0f32; VEL_RING_SIZE],
            steps: 0u32,
            last_directions: [0f32; 6],
            last_extremes: [[0f32;6];6],
            last_velocity: 0f32,
            last_diff: 0f32,
            last_match: -1,
        }
    }

    /// Adds a filtered sample to the accel ring and vel ring buffer
    /// 1162 cycles max each 20 ms / 50 Hz consumes max 58100 cycles per sec
    pub fn add_sample(&mut self, x_g: f32, y_g: f32, z_g: f32) {
        let mut current_accel = [0f32; 3];
        current_accel[0] = x_g;
        current_accel[1] = y_g;
        current_accel[2] = z_g;

        self.accel_ring_counter = self.accel_ring_counter.wrapping_add(1);
        self.accel_ring_x[self.accel_ring_counter % ACCEL_RING_SIZE] = current_accel[0];
        self.accel_ring_y[self.accel_ring_counter % ACCEL_RING_SIZE] = current_accel[1];
        self.accel_ring_z[self.accel_ring_counter % ACCEL_RING_SIZE] = current_accel[2];

        let mut world_z = [0f32; 3];
        let mut min = self.accel_ring_counter;
        if  ACCEL_RING_SIZE < self.accel_ring_counter {
            min = ACCEL_RING_SIZE;
        }

        world_z[0] = Filter::sum(&self.accel_ring_x) / min as f32;
        world_z[1] = Filter::sum(&self.accel_ring_y) / min as f32;
        world_z[2] = Filter::sum(&self.accel_ring_z) / min as f32;
    
        let normalization_factor = Filter::norm(&world_z);

        world_z[0] = world_z[0] / normalization_factor;
        world_z[1] = world_z[1] / normalization_factor;
        world_z[2] = world_z[2] / normalization_factor;

        let current_z = Filter::dot(&world_z, &current_accel) - normalization_factor;
        self.vel_ring_counter = self.vel_ring_counter.wrapping_add(1);
        
        self.vel_ring[self.vel_ring_counter % VEL_RING_SIZE] = current_z;


        let velocity_estimate = Filter::sum(&self.vel_ring);
        if self.accel_ring_counter % THRESHOLD_SIZE == 0 {
            self.calc_min_max();
        }
        self.vel_estimate_samples[self.accel_ring_counter % ACCEL_RING_SIZE] = velocity_estimate;
    }

    /// Determines from the values on the buffer is a step has been taken
    pub fn detect_step(&mut self) -> (f32, f32, f32,  bool) {
        let mut is_step = false;
        let mut last = 0.0;
        let current_velocity = self.vel_estimate_samples[self.accel_ring_counter % ACCEL_RING_SIZE];
       
        // Check if current velocity is larger or smaller then last to determine the direcion of the slope
        let mut direction = 0.0;
        if current_velocity > self.last_velocity {
            direction = 1.0;
        } else if current_velocity < self.last_velocity {
            direction = -1.0;
        }

        let k: usize = 0;

        if direction == - self.last_directions[k] {
            let mut ext_type = 1;
            if direction > 0.0 {
                ext_type = 0;
            }
            self.last_extremes[ext_type][k] = self.last_velocity;
            let mut  diff = self.last_extremes[ext_type][k] - self.last_extremes[1 - ext_type][k];
            if diff < 0.0 {
                diff = diff * -1.0;
            }

            if diff > self.step_threshold {
            
                let is_almost_as_large_as_previous = diff > (self.last_diff*1.5/3.0);
                let is_previous_large_enough = self.last_diff > (diff/4.0);
                let is_not_contra = self.last_match != 1 - (ext_type as i8);

                if is_almost_as_large_as_previous && is_previous_large_enough && is_not_contra {            
                    self.last_match = ext_type as i8;
                    is_step = true;
                } else {
                    self.last_match = -1;
                }
            }
            last = self.last_diff;
            self.last_diff = diff;
            
        }
        self.last_directions[k] = direction;
        self.last_velocity = current_velocity;
 
        (self.vel_estimate_samples[self.accel_ring_counter % ACCEL_RING_SIZE], self.last_diff, last, is_step)
    }
    
    pub fn calc_min_max(&mut self) {
        let mut min = 100.0;
        let mut max = 0.0;
        for value in self.vel_estimate_samples.iter() {
            if *value < min {
                min = *value;
            } else if *value > max {
                max = *value;
            }
        }
        self.step_threshold = (max - min) / THRESHOLD_DIVITER;
        if self.step_threshold < self.min_threshold {
            self.step_threshold = self.min_threshold;
        }
    }
    pub fn add_step(&mut self) {
        self.steps = self.steps.wrapping_add(1);
    }

    pub fn get_steps(&self) -> u32 {
        self.steps
    }

    pub fn get_threshold(&self) -> f32 {
        self.step_threshold
    }

}