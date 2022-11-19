#[derive(Clone, Copy)]
pub struct Systems {
    num_persons: f64, // Total number of persons onboard (crew + passengers)
    n_c: f64, // Number of crew
    v_pressurized: f64, // Volume of pressurized sections (ft3)
    w_cargo: f64, // Maximum cargo weight (lb)
    w_uav: f64, // Uninstalled avionics weight; typically 800 − 1400 (lb)
    fwa: f64, // Fuselage wetted area (ft2)
    num_seats: f64, // Number of seats of given type
    w_seats: f64, // Weight of single seat (lb); ≈ 60 for flight deck seats, 32 for passenger seats, and 11 for troop seats
    k_lav: f64, // 1.11 for long range aircraft; 0.31 for short range aircraft; 3.9 for business jets.
    k_buf: f64, // 1.02 for short ranges; 5.68 for very long ranges
    r_kva: f64, // System electrical rating; typically 40 − 60 for transports (kVA)
    l_a: f64, // Electrical routing distance; generators to avionics to cockpit (ft)
    n_gen: f64, // Number of generators; typically = Nen
    n_f: f64, // Number of functions performed by controls; typically 4 − 7
    l_f: f64, // Total fuselage length (ft)
    b_w: f64, // Wing span (ft)
    k_r: f64, // 1.133 for reciprocating engines; 1.0 otherwise
    n_en: f64, // number of engines
    w_apu: f64, // Uninstalled APU weight (lb)
    s_cs: f64, // Total control surface area (ft2)
    i_y: f64, // Pitching moment of inertia; ≈ WoKy^2 (lb ft^2)
    n_m: f64, // Number of mechanical function performed by controls; typically 0 − 2
}

impl Systems {
    pub fn new() -> Self {
        Self {
            num_persons: 9.90,
            n_c: 9.90,
            v_pressurized: 9.90,
            w_cargo: 9.90,
            w_uav: 9.90,
            fwa: 9.90,
            num_seats: 9.90,
            w_seats: 9.90,
            k_lav: 9.90,
            k_buf: 9.90,
            r_kva: 9.90,
            l_a: 9.90,
            n_gen: 9.90,
            n_f: 9.90,
            l_f: 9.90,
            b_w: 9.90,
            k_r: 9.90,
            n_en: 9.90,
            w_apu: 9.90,
            s_cs: 9.90,
            i_y: 9.90,
            n_m: 9.90,
        }
    }

    /// - w_dg: design gross weight in lb
    pub fn weight(self, w_dg: f64) -> f64 {
        let r = self.weight_flight_controls() +
            self.weight_installed_apu() +
            self.weight_instruments() +
            self.weight_hydraulic_system() +
            self.weight_electrical_system() +
            self.weight_avionics() +
            self.weight_furnishings() +
            self.weight_ac() +
            self.weight_anti_icing_system(w_dg) +
            self.weight_handling_gear(w_dg);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_flight_controls(self) -> f64 {
        // numerator
        let mut r = 145.9;
        r *= self.n_f.powf(0.554);
        r *= self.s_cs.powf(0.2);
        r *= f64::powf(self.i_y * 1e-6, 0.07);
        // denominator
        r /= 1. + self.n_m / self.n_f;
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_installed_apu(self) -> f64 {
        let r = 2.2 * self.w_apu;
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_instruments(self) -> f64 {
        let mut r = 4.509;
        r *= self.k_r;
        r *= self.n_c.powf(0.541);
        r *= self.n_en;
        r *= f64::powf(self.l_f + self.b_w, 0.5);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_hydraulic_system(self) -> f64 {
        let mut r = 0.2673;
        r *= self.n_f;
        r *= f64::powf(self.l_f + self.b_w, 0.937);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_electrical_system(self) -> f64 {
        let mut r = 7.291;
        r *= self.r_kva.powf(0.782);
        r *= self.l_a.powf(0.346);
        r *= self.n_gen.powf(0.1);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_avionics(self) -> f64 {
        let r = 1.73 * self.w_uav.powf(0.983);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_furnishings(self) -> f64 {
        let mut r = 0.0577 * self.n_c.powf(0.1)
            * self.w_cargo.powf(0.393) * self.fwa.powf(0.75);
        r += self.num_seats * self.w_seats;
        r += self.k_lav * self.num_persons.powf(1.33);
        r += self.k_buf * self.num_persons.powf(1.12);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    fn weight_ac(self) -> f64 {
        let mut r = 62.36;
        r *= self.num_persons.powf(0.25);
        r *= f64::powf(self.v_pressurized * 0.001, 0.604);
        r *= self.w_uav.powf(0.1);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    /// - w_dg: design gross weight in lb
    fn weight_anti_icing_system(self, w_dg: f64) -> f64 {
        let r = 0.002 * w_dg;
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    /// - w_dg: design gross weight in lb
    fn weight_handling_gear(self, w_dg: f64) -> f64 {
        let r = 0.0003 * w_dg;
        if r < 0. { eprintln!("negative weight"); }
        r
    }
}
