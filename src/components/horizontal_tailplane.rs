pub struct HorizontalTailplane {
    k_uht: f64,    // 1.143 for all-moving tail, 1.0 otherwise
    s_ht: f64,     // horizontal tailplane area in ft^2
    k_y: f64,      // aircraft pitching radius of giration, approx 0.3L_ht in ft
    ar_h: f64,     // Horizontal tailplane aspect ratio
    s_e: f64,      // Elevator area (ft2)
    f_w: f64,      // Fuselage width at horizontal tail intersection (ft)
    b_h: f64,      // Horizontal tailplane b_h (ft)
    l_ht: f64, // l_ht from wing aerodynamic centre to horizontal tailplane aerodynamic centre (ft)
    sweep_ht: f64, // Horizontal tailplane quarter chord sweep_ht in radians
}

impl HorizontalTailplane {
    pub fn new() -> Self {
        Self {
            k_uht: 1.0,
            s_ht: 9.90,
            k_y: 9.90,
            ar_h: 9.90,
            s_e: 9.90,
            f_w: 9.90,
            b_h: 9.90,
            l_ht: 9.90,
            sweep_ht: 9.90,
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        // numerator
        let mut r = 0.0379;
        r *= self.k_uht;
        r *= f64::powf(w_dg, 0.639);
        r *= f64::powf(n_z, 0.1);
        r *= f64::powf(self.s_ht, 0.75);
        r *= f64::powf(self.k_y, 0.704);
        r *= f64::powf(self.ar_h, 0.166);
        r *= f64::powf(1.0 + self.s_e / self.s_ht, 0.1);
        // denominator
        r /= f64::powf(1. + self.f_w / self.b_h, 0.25);
        r /= self.l_ht;
        r /= f64::cos(self.sweep_ht);
        if r < 0. {
            eprintln!("negative weight");
        }
        r
    }
}
