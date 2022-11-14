pub struct Wings {
    s_ref: f64, // reference wing area in ft^2
    ar: f64, // wing aspect ratio
    taper_ratio: f64, // wing taper ratio
    s_cs: f64, // area of wing mounted control surfaces in ft^2
    sweep: f64, // wing quarter chord sweep in rad
    t_c_ratio: f64, // wing root thickness to chord ratio
}

impl Wings {
    pub fn new() -> Self {
        Self {
            s_ref: 9.90,
            ar: 9.90,
            taper_ratio: 9.90,
            s_cs: 9.90,
            sweep: 9.90,
            t_c_ratio: 9.90,
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        // numerator
        let mut r: f64 = 0.0051;
        r *= f64::powf(w_dg * n_z, 0.557);
        r *= self.s_ref.powf(0.649);
        r *= self.ar.powf(0.5);
        r *= f64::powf(1.0 + self.taper_ratio, 0.1);
        r *= self.s_cs.powf(0.1);
        // denominator
        r /= self.sweep.cos();
        r /= self.t_c_ratio.powf(0.4);
        r
    }
}