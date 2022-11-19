pub struct VerticalTailplace {
    h_t_over_h_v: f64,// Location of horizontal tailplane on vertical tail. 0.0 for fuselage mounted horizontal tail; 1.0 for T-tail
    s_vt: f64, // Vertical tailplane area (ft2)
    k_z: f64, // Aircraft yaw radius of gyration approx L_vt (ft)
    ar_v: f64, // Vertical tailplane aspect ratio
    l_vt: f64, // Length from wing aerodynamic centre to vertical tailplane aerodynamic centre (ft)
    sweep_vt: f64, // Vertical tailplane quarter chord sweep
    t_c_ratio_root_v: f64, // Vertical tailplane root thickness to chord ratio
}

impl VerticalTailplace {
    pub fn new() -> Self {
        Self {
            h_t_over_h_v: 1.0,
            s_vt: 9.90,
            k_z: 9.90,
            ar_v: 9.90,
            l_vt: 9.90,
            sweep_vt: 9.90,
            t_c_ratio_root_v: 9.90,
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        // numerator
        let mut r = 0.0026;
        r *= f64::powf(1. + self.h_t_over_h_v, 0.225);
        r *= w_dg.powf(0.556);
        r *= n_z.powf(0.536);
        r *= self.s_vt.powf(0.5);
        r *= self.k_z.powf(0.875);
        r *= self.ar_v.powf(0.35);
        // denominator
        r /= self.l_vt.powf(0.5);
        r /= self.sweep_vt.cos();
        r /= self.t_c_ratio_root_v.powf(0.5);
        if r < 0. { eprintln!("negative weight"); }
        r
    }
}