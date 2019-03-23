use libm::{F32Ext, F64Ext};

pub fn to_celsius(sample: u16) -> f32 {
    let volts: f32 = bits_to_volts(sample);
    let ohms: f32 = volts_to_ohms(volts);
    let temp: f32 = ohms_to_celsius(ohms);

    let decimals: u8 = 1;
    let temp_trunc: f32 = truncate_f32(temp, decimals);
    return temp_trunc;
}

// bits_to_volts: From 12-bit ADC read sample to voltage over Rx, the pt1000 resistor.
fn bits_to_volts(sample: u16) -> f32 {
    let sample: f32 = sample as f32; // Cast to floating point.
    let v: f32 = 3.30; // Volts. Supply voltage, reference voltage.
    let max_adc: f32 = 4095.0; // 12-bit ADC.
    let g: f32 = 29.41; // Volts per volt. Gain of instrumentation amplifier.

    let vt: f32 = (sample * v / max_adc) / g;
    
    return vt;
}

// volts_to_ohms: Calculate resistance (Ohms) of the pt1000 based on (unamplified) voltage.
fn volts_to_ohms(volts: f32) -> f32 {
    let v: f32 = 3.30; // Volts. Supply voltage, reference voltage.

    // Wheatstone Bridge circuit resistors.
    let r: f32 = 7.9583e+3;
    let r3: f32 = 1.8e+3;

    let ohms: f32 = r / ((r / r3 + 1.0) / (volts / v * (r / r3 + 1.0) + 1.0) - 1.0) - 1000.0;
    return ohms;
}

// ohms_to_celsius: Implements EN 60 751 standard for Pt1000 resistors.
fn ohms_to_celsius(r: f32) -> f32 {
    // Constants:
    let a: f32 = 3.90802e-3;     // Celsius^-1
    let b: f32 = - 5.775e-7;     // Celsius^-2
    let c: f32 = - 4.2735e-12;   // Celsius^-4
    let r0: f32 = 1000.0;          // Ohms. Pt1000: 1kOhm at 0 C.

    let tolerance: f32 = 0.1;    // Celsius. Stop iterating when converging within this.
    let mut t: f32 = -5.0;             // Celsius. Initial guess for negative temperatures.
    let mut dt: f32 = 100.0;
    if r < r0 {
        let tmp: f32 = (r * a).powf(2.0) - 4.0 * b * (r0 - r);
        t = (-r0 * a + tmp.sqrt() / (2.0 * r0 * b));
    } else {
        while tolerance < dt {
            let tn: f32 = t - (t2r(t) - r) / t2r_prime(t);
            dt = tn - t;
            t = tn;
        }
    }
    return t;
}

// t2r: Temperature to resistance. Implements EN 60 751 standard for Pt1000
// resistors. Piecewise function of polynomial fits. Used for iterating 
// negative temperatures.
fn t2r(t: f32) -> f32 {
    // Constants:
    let a: f32 = 3.90802e-3;     // Celsius^-1
    let b: f32 = - 5.775e-7;     // Celsius^-2
    let c: f32 = - 4.2735e-12;   // Celsius^-4
    let r0: f32 = 1000.0;          // Ohms. Pt1000: 1kOhm at 0 C.

    let mut r: f32 = 0.0;
    if t < 0.0 {
        r = r0 * (1.0 + a * t + b * t.powf(2.0) + c * (t - 100.0) * t.powf(3.0));
    } else {
        r = r0 * (1.0 + a * t + b * t.powf(2.0));
    }
    return r;
}

// r_prime: Derivative of r.
fn t2r_prime(t: f32) -> f32 {
    // Constants:
    let a: f32 = 3.90802e-3;     // Celsius^-1
    let b: f32 = - 5.775e-7;     // Celsius^-2
    let c: f32 = - 4.2735e-12;   // Celsius^-4
    let r0: f32 = 1000.0;          // Ohms. Pt1000: 1kOhm at 0 C.

    let mut r_prime: f32 = 0.0;
    if t < 0.0 {
        r_prime = r0 * (a + 2.0 * b * t + 4.0 * c * t.powf(3.0) - 100.0 * c * 3.0 * t.powf(2.0));
    } else {
        r_prime = r0 * (a + 2.0 * b * t);
    }
    return r_prime;
}

// truncate_f32: Remove digits after "d" decimals.
fn truncate_f32(fp: f32, d: u8) -> f32 {
    let mut tmp: f32 = fp * 10f32.powf(d as f32);   // Move decimal sign.
    tmp -= tmp % 1f32;                              // Remove digits after decimal sign.
    tmp = tmp.trunc();
    tmp /= 10f32.powf(d as f32);         // Move decimal sign back.
    return tmp;
}