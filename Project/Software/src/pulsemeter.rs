use crate::time;

pub struct Pulse {
    pub pulse: f32,     // Current pulse, beats per minute.
    pub counts: u8,
    samples: [u16; 80], // Buffer: last few ADC values. 80 / 16 Hz = 5 s.
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
            samples: [0; 80],
            frequency: frequency,
            min: 0,
            max: 0,
            ratio: 1.0,
            th_high: 0.7,
            th_low: 0.3,
        }
    }

    pub fn write_sample(&mut self, sample: u16) {
        self.samples.rotate_left(1);
        self.samples[80 - 1] = sample;
    }

    pub fn update(&mut self) {
        self.new_max();
        self.new_min();
        self.new_ratio();

        // Number of peaks
        let peaks: u8 = self.count_peaks();

        
        // Time frame for "samples"
        let ts: f32 = 1.0 / (self.frequency.0 as f32);
        let t: f32 = (self.samples.len() as f32) * ts;
        
        // Take average to not "trust" the recent measurement too much.
        self.pulse = (self.pulse + 60.0 * (peaks as f32)/ t) / 2.0;
        self.counts = peaks;
    }

    pub fn new_max(&mut self) {
        let mut max: u16 = 0;
        for s in self.samples.iter() {
            if *s >= max {
                max = *s;
            }
        }
        self.max = max;
    }

    pub fn new_min(&mut self) {
        let mut min: u16 = 65535;
        for s in self.samples.iter() {
            if *s <= min {
                min = *s;
            }
        }
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
        let m = self.min;
        let r = self.ratio;
        let hi = self.th_high;
        let lo = self.th_low;

        let mut is_high: bool = false;

        if self.samples[0] as f32 / m as f32 >= hi * r {
            is_high = true;
        }

        for s in self.samples.iter() {
            // Current sample as ratio to minimum.
            let r_s = (*s as f32) / (m as f32);
            // HYSTERESIS
            is_high = match is_high {
                true => {               // It was high.
                    let mut tmp = true;
                    if r_s < lo * r {   // Has it gone low?
                        tmp = false;    // ... yes.
                    } else {
                        tmp = true;     // ... no.
                    }
                    tmp
                },
                false => {              // It was low.
                    let mut tmp = false;
                    if r_s > hi * r {   // Has it gone high?
                        tmp = true;     // ... yes,
                        peaks += 1;     // so count this one.
                    } else {
                        tmp = false;    // ... no.
                    }
                    tmp
                },
            };
        }
        return peaks;
    }
}
