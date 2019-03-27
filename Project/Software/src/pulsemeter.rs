use crate::time;

pub struct Pulse {
    pub pulse: f32,     // Current pulse, beats per minute.
    pub counts: u8,
    pub samples: [u16; 800], // Buffer: last few ADC values. 400 / 32 Hz = 12.5 s.
    frequency: time::Hertz, // Hz. Samples saved per second.
    pub min: u16,       // Minimum measured recent adc value.
    pub max: u16,       // Maximum measured recent adc value.
    pub ratio: f32,     // A ratio describing max-to-min difference.
    pub th_high: f32,   // Hysteresis high threshold. 0 to 1 decimal value. Multiplied by "ratio".
    pub th_low: f32,    // Hysteresis low threshold. 0 to 1 decimal value. Multiplied by "ratio".
}

impl Pulse {
    pub fn new(frequency: time::Hertz) -> Self {
        Pulse {
            counts: 0,
            pulse: 50.0,
            samples: [0; 800],
            frequency: frequency,
            min: 0,
            max: 0,
            ratio: 1.0,
            th_high: 0.7,
            th_low: 0.4,
        }
    }

    pub fn write_sample(&mut self, sample: u16) {
        self.samples.rotate_left(1);
        self.samples[self.samples.len() - 1] = sample;
    }

    pub fn read(&self) -> f32 {
        self.pulse
    }

    // Consumes around 40k cycles
    pub fn update(&mut self) {
        self.new_max_min();
        self.new_ratio();

        // Number of peaks
        let peaks: u8 = self.count_peaks();

        if peaks != 0 {
            // Time frame for "samples"
            let ts: f32 = 1.0 / (2.0 * (self.frequency.0 as f32));
            let t: f32 = (self.samples.len() as f32) * ts;
            
            // Trust previous value more: weights 5/6 and 1/6.
            self.pulse = (5.0 * self.pulse + 60.0 * (peaks as f32)/ t) / 6.0;
        }
        self.counts = peaks;
    }

    pub fn new_max_min(&mut self) {
        let mut max: u16 = 0;
        let mut min: u16 = 4095;

        for s in self.samples.iter() {
            if *s >= max {
                max = *s;
            } else if *s <= min {
                min = *s;
            }
        }
        self.max = max;
        self.min = min;
    }

    pub fn new_ratio(&mut self) {
        let mut ratio: f32 = 4.0;
        if self.min != 0 {
            ratio = (self.max as f32) / (self.min as f32);
        }
        self.ratio = ratio;
    }

    pub fn count_peaks(&mut self) -> u8 {
        let mut peaks: u8 = 0;
        let d = (self.max - self.min) as f32;
        let r = self.ratio;
        let hi = self.th_high;
        let lo = self.th_low;

        let mut is_high: bool = false;

        if ((self.samples[0] - self.min) as f32) / d >= hi {
            is_high = true;
        }

        for s in self.samples.iter() {
            let r_s = ((*s - self.min) as f32) / d;
            // HYSTERESIS
            
            is_high = match is_high {
                true => {               // It was high.
                    if r_s < lo {       // Has it gone low?
                        false   // ... yes.
                    } else {
                        true     // ... no.
                    }
                },
                false => {              // It was low.
                    if r_s > hi { // Has it gone high?
                        peaks += 1;     // so count this one.
                        true
                    } else {
                        false
                    }
                },
            };
        }
        return peaks;
    }
}
