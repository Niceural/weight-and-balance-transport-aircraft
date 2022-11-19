#[derive(Clone, Copy)]
pub struct Engines {
    // nacelle
    k_ng: f64, // 1.017 for pylon mounted nacelle, 1.0 otherwise
    n_lt: f64, // Nacelle length (ft)
    n_w: f64, // Nacelle width (ft)
    w_enc: f64, // Weight of engine and contents in lb (see pdf)
    n_en: f64, // Number of engines
    s_n: f64, // Nacelle wetted area (ft^2)
    // engine controls
    l_ec: f64, // Engine controls routing distance; engine to cockpit - total if multiengine (ft)
    // engine pneumatic starters
    w_en: f64, // Engine weight (lb)
    // fuel system
    v_t: f64, // Total volume of fuel tanks (gal)
    n_t: f64, // Total number of fuel tanks
    v_p: f64, // Self sealing tank volume (gal)
    v_i: f64, // Integral fuel tank volume (gal)
}

impl Engines {
    pub fn new() -> Self {
        Self {
            // nacelle
            k_ng: 9.90,
            n_lt: 9.90,
            n_w: 9.90,
            w_enc: 9.90,
            n_en: 9.90,
            s_n: 9.90,
            // engine controls
            l_ec: 9.90,
            // engine pneumatic starters
            w_en: 9.90,
            // fuel system
            v_t: 9.90,
            n_t: 9.90,
            v_p: 9.90,
            v_i: 9.90,
        }
    }

    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(self, n_z: f64) -> f64 {
        let r = self.weight_nacelle(n_z) +
            self.weight_engine_controls() +
            self.weight_engine_pneumatic_starter() +
            self.weight_fuel_system();
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    /// - n_z: ultimate load factor, 1.5x limit load factor
    fn weight_nacelle(self, n_z: f64) -> f64 {
        let mut r: f64 = 0.6724;
        r *= self.k_ng;
        r *= f64::powf(self.n_lt, 0.1);
        r *= f64::powf(self.n_w, 0.294);
        r *= f64::powf(n_z, 0.119);
        r *= f64::powf(self.w_enc, 0.611);
        r *= f64::powf(self.n_en, 0.984);
        r *= f64::powf(self.s_n, 0.224);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_engine_controls(self) -> f64 {
        let r = 5. * self.n_en + 0.8 * self.l_ec;
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_engine_pneumatic_starter(self) -> f64 {
        let mut r = 49.19;
        r *= f64::powf(self.n_en * self.w_en * 1e-3, 0.541);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_fuel_system(self) -> f64 {
        let mut r = 2.405;
        r *= self.v_t.powf(0.606);
        r *= self.n_t.powf(0.5);
        r *= 1. + self.v_p / self.v_t;
        r /= 1. + self.v_i / self.v_t;
        if r < 0. { eprintln!("negative weight"); }
        r
    }
}