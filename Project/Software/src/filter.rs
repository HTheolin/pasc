use libm::F32Ext;

pub struct Filter {}

impl Filter {

    pub fn sum(array: &[f32]) -> f32 {
        let mut sum: f32 = 0.0;
        for i in array {
            sum += i;
        }
        sum
    }
 
    pub fn cross(array_a: &[f32], array_b: &[f32]) -> [f32; 3] {
        let mut cross = [0f32; 3];
        cross[0] = array_a[1] * array_b[2] - array_a[2] * array_b[1];
        cross[1] = array_a[2] * array_b[0] - array_a[0] * array_b[2];
        cross[2] = array_a[0] * array_b[1]  - array_a[1] * array_b[0];
        cross
    }
 
    pub fn norm(array: &[f32]) -> f32 {
        let mut norm: f32 = 0.0;
        for i in array {
            norm += i*i;
        }
        norm = norm.sqrt();
        norm
    }

    pub fn dot(a: &[f32], b: &[f32]) -> f32 {
        let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        dot
    }
}