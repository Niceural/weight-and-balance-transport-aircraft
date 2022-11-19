#[derive(Copy, Clone)]
pub struct LandingGear {
    // main landing gear
    k_mp: f64, // 1.126 for kneeling main gear; 1.0 otherwise
    l_m: f64, // Main landing gear length (inches)
    n_mw: f64, // Number of main wheels
    v_s: f64, // Landing stall speed (ft/s)
    n_mss: f64, // Number of main gear shock struts
    // nose landing gear
    k_np: f64, // 1.15 for kneeling nose-gear; 1.0 otherwise
    l_n: f64, // Nose landing gear length (inches)
    n_nw: f64, // Number of nose wheels
    // both
    w_l: f64, // Landing design gross weight (lb)
    n_l: f64, // Ultimate landing gear load factor. 1.5 × N_gear
}

impl LandingGear {
    pub fn new() -> Self {
        Self {
            // main landing gear
            k_mp: 9.90,
            l_m: 9.90,
            n_mw: 9.90,
            v_s: 9.90,
            n_mss: 9.90,
            // nose landing gear
            k_np: 9.90,
            l_n: 9.90,
            n_nw: 9.90,
            // both
            w_l: 9.90,
            n_l: 9.90,
        }
    }

    pub fn weight(self) -> f64 {
        let r = self.weight_main_landing_gear() + 
            self.weight_nose_landing_gear();
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_main_landing_gear(self) -> f64 {
        let mut r = 0.0106;
        r *= self.k_mp;
        r *= self.w_l.powf(0.888);
        r *= self.n_l.powf(0.25);
        r *= self.l_m.powf(0.4);
        r *= self.n_mw.powf(0.321);
        r *= self.v_s.powf(0.1);
        r /= self.n_mss.powf(0.5);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_nose_landing_gear(self) -> f64 {
        let mut r = 0.032;
        r *= self.k_np;
        r *= self.w_l.powf(0.646);
        r *= self.n_l.powf(0.2);
        r *= self.l_n.powf(0.5);
        r *= self.n_nw.powf(0.45);
        if r < 0. { eprintln!("negative weight"); }
        r
    }
}