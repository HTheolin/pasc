const ACCEL_RING_SIZE: usize = 50;
const VEL_RING_SIZE: usize = 10;
const STEP_DELAY_MS: u64 = 250;

use crate::filter::Filter;
pub struct Step {
    step_treshold: f32,
    accel_ring_counter: usize,
    accel_ring_x: [f32; ACCEL_RING_SIZE],
    accel_ring_y: [f32; ACCEL_RING_SIZE],
    accel_ring_z: [f32; ACCEL_RING_SIZE],
    vel_ring_counter: usize,
    vel_ring: [f32; VEL_RING_SIZE],
    old_velocity_estimate: f32,
    steps: u32,
}

impl Step {
    pub fn new(threshold: f32) -> Self {
        Step {
            step_treshold: threshold,
            accel_ring_counter: 0usize,
            accel_ring_x: [0f32; ACCEL_RING_SIZE],
            accel_ring_y: [0f32; ACCEL_RING_SIZE],
            accel_ring_z: [0f32; ACCEL_RING_SIZE],
            vel_ring_counter: 0usize,
            vel_ring: [0f32; VEL_RING_SIZE],
            old_velocity_estimate: 0f32,
            steps: 0u32,
        }
    }

    pub fn calc_step(&mut self, x_g: f32, y_g: f32, z_g: f32) -> (f32, bool) {
        let mut step = false;
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
 
        if velocity_estimate > self.step_treshold && self.old_velocity_estimate <= self.step_treshold {
            step = true;
            self.steps = self.steps.wrapping_add(1);
        }
        self.old_velocity_estimate = velocity_estimate;
        (velocity_estimate, step)
    }

    pub fn add_step(&mut self) {
        self.steps = self.steps.wrapping_add(1);
    }

    pub fn get_steps(&self) -> u32 {
        self.steps
    }

}