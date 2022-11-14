pub struct Systems {
    num_persons: f64, // Total number of persons onboard (crew + passengers)
    num_crew: f64, // Number of crew
    v_pressurized: f64, // Volume of pressurized sections (ft3)
    w_cargo: f64, // Maximum cargo weight (lb)
    w_uav: f64, // Uninstalled avionics weight; typically 800 − 1400 (lb)
    fwa: f64, // Fuselage wetted area (ft2)
}

impl Systems {
    pub fn new() -> Self {
        Self {
            num_persons: 9.90,
            num_crew: 9.90,
            v_pressurized: 9.90,
            w_cargo: 9.90,
            w_uav: 9.90,
            fwa: 9.90,
        }
    }

    /// - w_dg: design gross weight in lb
    pub fn weight(self, w_dg: f64) -> f64 {
        let mut r = 0.0;
        r += self.weight_handling_gear(w_dg);
        // r += self.weight_anti_icing_system(w_dg);
        // r += self.weight_ac();
        if r < 0. { panic!("negative weight!!!"); }
        r
    }

    fn weight_furnishings(self) -> f64 {
        // first term
        let mut r = 0.0577;
        r *= self.num_crew.powf(0.1);
        r *= self.w_cargo.powf(0.393);
        r *= self.fwa.powf(0.75);
        // second term
        if r < 0. { panic!("negative weight!!!"); }
        r
    }

    fn weight_ac(self) -> f64 {
        let mut r = 62.36;
        r *= self.num_persons.powf(0.25);
        r *= f64::powf(self.v_pressurized * 0.001, 0.604);
        r *= self.w_uav.powf(0.1);
        if r < 0. { panic!("negative weight!!!"); }
        r
    }

    /// - w_dg: design gross weight in lb
    fn weight_anti_icing(self, w_dg: f64) -> f64 {
        let r = 0.002 * w_dg;
        if r < 0. { panic!("negative weight!!!"); }
        r
    }

    /// - w_dg: design gross weight in lb
    fn weight_handling_gear(self, w_dg: f64) -> f64 {
        let r = 0.0003 * w_dg;
        if r < 0. { panic!("negative weight!!!"); }
        r
    }
}
