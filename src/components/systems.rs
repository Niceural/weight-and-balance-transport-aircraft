use crate::Params;

#[derive(Clone, Copy)]
pub struct Systems {
    n_p: f64, // Total number of persons onboard (crew + passengers)
    n_c: f64, // Number of crew
    v_pr: f64, // Volume of pressurized sections (ft3)
    w_c: f64, // Maximum cargo weight (lb)
    w_uav: f64, // Uninstalled avionics weight; typically 800 − 1400 (lb)
    s_f: f64, // Fuselage wetted area (ft2)
    n_seat: f64, // Number of seats of given type
    w_seat: f64, // Weight of single seat (lb); ≈ 60 for flight deck seats, 32 for passenger seats, and 11 for troop seats
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
    pub fn new(params: &Params) -> Self {
        Self {
            n_p: params.get("n_p").unwrap().clone(),
            n_c: params.get("n_c").unwrap().clone(),
            v_pr: params.get("v_pr").unwrap().clone(),
            w_c: params.get("w_c").unwrap().clone(),
            w_uav: params.get("w_uav").unwrap().clone(),
            s_f: params.get("s_f").unwrap().clone(),
            n_seat: params.get("n_seat").unwrap().clone(),
            w_seat: params.get("w_seat").unwrap().clone(),
            k_lav: params.get("k_lav").unwrap().clone(),
            k_buf: params.get("k_buf").unwrap().clone(),
            r_kva: params.get("r_kva").unwrap().clone(),
            l_a: params.get("l_a").unwrap().clone(),
            n_gen: params.get("n_gen").unwrap().clone(),
            n_f: params.get("n_f").unwrap().clone(),
            l_f: params.get("l_f").unwrap().clone(),
            b_w: params.get("b_w").unwrap().clone(),
            k_r: params.get("k_r").unwrap().clone(),
            n_en: params.get("n_en").unwrap().clone(),
            w_apu: params.get("w_apu").unwrap().clone(),
            s_cs: params.get("s_cs").unwrap().clone(),
            i_y: params.get("i_y").unwrap().clone(),
            n_m: params.get("n_m").unwrap().clone(),
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
            self.weight_air_conditioning() +
            self.weight_anti_icing_system(w_dg) +
            self.weight_handling_gear(w_dg);
        if r < 0. { panic!("negative weight"); }
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
        if r < 0. { panic!("negative weight"); }
        r
    }

    fn weight_installed_apu(self) -> f64 {
    }

    fn weight_instruments(self) -> f64 {
        let mut r = 4.509;
        r *= self.k_r;
        r *= self.n_c.powf(0.541);
        r *= self.n_en;
        r *= f64::powf(self.l_f + self.b_w, 0.5);
        if r < 0. { panic!("negative weight"); }
        r
    }

    fn weight_hydraulic_system(self) -> f64 {
        let mut r = 0.2673;
        r *= self.n_f;
        r *= f64::powf(self.l_f + self.b_w, 0.937);
        if r < 0. { panic!("negative weight"); }
        r
    }

    fn weight_electrical_system(self) -> f64 {
        let mut r = 7.291;
        r *= self.r_kva.powf(0.782);
        r *= self.l_a.powf(0.346);
        r *= self.n_gen.powf(0.1);
        if r < 0. { panic!("negative weight"); }
        r
    }

    fn weight_avionics(self) -> f64 {
        let r = 1.73 * self.w_uav.powf(0.983);
        if r < 0. { panic!("negative weight"); }
        r
    }

    fn weight_furnishings(self) -> f64 {
        let mut r = 0.0577 * self.n_c.powf(0.1)
            * self.w_c.powf(0.393) * self.s_f.powf(0.75);
        r += self.n_seat * self.w_seat;
        r += self.k_lav * self.n_p.powf(1.33);
        r += self.k_buf * self.n_p.powf(1.12);
        if r < 0. { panic!("negative weight"); }
        r
    }

    fn weight_air_conditioning(self) -> f64 {
        let mut r = 62.36;
        r *= self.n_p.powf(0.25);
        r *= f64::powf(self.v_pr * 0.001, 0.604);
        r *= self.w_uav.powf(0.1);
        if r < 0. { panic!("negative weight"); }
        r
    }

    /// - w_dg: design gross weight in lb
    fn weight_anti_icing_system(self, w_dg: f64) -> f64 {
        let r = 0.002 * w_dg;
        if r < 0. { panic!("negative weight"); }
        r
    }

    /// - w_dg: design gross weight in lb
    fn weight_handling_gear(self, w_dg: f64) -> f64 {
        let r = 0.0003 * w_dg;
        if r < 0. { panic!("negative weight"); }
        r
    }
}

//------------------------------------------ flight controls

struct FlightControls {
    n_f: f64,
    s_cs: f64,
    i_y: f64,
    n_m: f64,
}

impl FlightControls {
    pub fn new(params: &Params) -> Self {
        Self {
            n_f: params.get("").unwrap().clone(),
            s_cs: params.get("").unwrap().clone(),
            i_y: params.get("").unwrap().clone(),
            n_m: params.get("").unwrap().clone(),
        }
    }

    pub fn weight(self) -> f64 {
        // numerator
        let mut r = 145.9;
        r *= self.n_f.powf(0.554);
        r *= self.s_cs.powf(0.2);
        r *= f64::powf(self.i_y * 1e-6, 0.07);
        // denominator
        r /= 1. + self.n_m / self.n_f;
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self) -> f64 {
        9.90
    }
}

//------------------------------------------ installed apu

struct InstalledApu {
    w_apu: f64,
}

impl InstalledApu {
    pub fn new(params: &Params) -> Self {
        Self {
            w_apu: params.get("w_apu").unwrap().clone(),
        }
    }

    pub fn weight(self) -> f64 {
        let r = 2.2 * self.w_apu;
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self) -> f64 {
        9.90
    }
}

//------------------------------------------ instruments

struct Instruments {
    k_r: f64,
    n_c: f64,
    n_en: f64,
    l_f: f64,
    b_w: f64,
}

impl Instruments {
    pub fn new(params: &Params) -> Self {
        Self {
            k_r: params.get("").unwrap().clone(),
            n_c: params.get("").unwrap().clone(),
            n_en: params.get("").unwrap().clone(),
            l_f: params.get("").unwrap().clone(),
            b_w: params.get("").unwrap().clone(),
        }
    }

    pub fn weight(self) -> f64 {
        let mut r = 4.509;
        r *= self.k_r;
        r *= self.n_c.powf(0.541);
        r *= self.n_en;
        r *= f64::powf(self.l_f + self.b_w, 0.5);
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self) -> f64 {
        9.90
    }
}

//------------------------------------------ hydrolic system

struct HydrolicSystem {
    n_f: f64,
    l_f: f64,
    b_w: f64,
}

impl HydrolicSystem {
    pub fn new(params: &Params) -> Self {
        Self {
            n_f: params.get("n_f").unwrap().clone(),
            l_f: params.get("l_f").unwrap().clone(),
            b_w: params.get("b_w").unwrap().clone(),
        }
    }

    pub fn weight(self) -> f64 {
        let mut r = 0.2673;
        r *= self.n_f;
        r *= f64::powf(self.l_f + self.b_w, 0.937);
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self) -> f64 {
        9.90
    }
}

//------------------------------------------ electrical systems

struct ElectricalSystems {
    r_kva: f64,
    l_a: f64,
    n_gen: f64,
}

impl ElectricalSystems {
    pub fn new(params: &Params) -> Self {
        Self {
            r_kva: params.get("r_kva").unwrap().clone(),
            l_a: params.get("l_a").unwrap().clone(),
            n_gen: params.get("n_gen").unwrap().clone(),
        }
    }

    pub fn weight(self) -> f64 {
        let mut r = 7.291;
        r *= self.r_kva.powf(0.782);
        r *= self.l_a.powf(0.346);
        r *= self.n_gen.powf(0.1);
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self) -> f64 {
        9.90
    }
}

//------------------------------------------ avionics

struct Avionics {
    w_uav: f64,
}

impl Avionics {
    pub fn new(params: &Params) -> Self {
        Self {
            w_uav: params.get("w_uav").unwrap().clone(),
        }
    }

    pub fn weight(self) -> f64 {
        let r = 1.73 * self.w_uav.powf(0.983);
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self) -> f64 {
        9.90
    }
}

//------------------------------------------ furnishing

struct Furnishing {
    n_c: f64,
    w_c: f64,
    s_f: f64,
    n_seat: f64,
    w_seat: f64,
    k_lav: f64,
    n_p: f64,
    k_buf: f64,
}

impl Furnishing {
    pub fn new(params: &Params) -> Self {
        Self {
            n_c: params.get("n_c").unwrap().clone(),
            w_c: params.get("w_c").unwrap().clone(),
            s_f: params.get("s_f").unwrap().clone(),
            n_seat: params.get("n_seat").unwrap().clone(),
            w_seat: params.get("w_seat").unwrap().clone(),
            k_lav: params.get("k_lav").unwrap().clone(),
            n_p: params.get("n_p").unwrap().clone(),
            k_buf: params.get("k_buf").unwrap().clone(),
        }
    }

    pub fn weight(self) -> f64 {
        let mut r = 0.0577 * self.n_c.powf(0.1)
            * self.w_c.powf(0.393) * self.s_f.powf(0.75);
        r += self.n_seat * self.w_seat;
        r += self.k_lav * self.n_p.powf(1.33);
        r += self.k_buf * self.n_p.powf(1.12);
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self) -> f64 {
        9.90
    }
}

//------------------------------------------ air conditioning

struct AirConditioning {
    n_p: f64,
    v_pr: f64,
    w_uav: f64,
}

impl AirConditioning {
    pub fn new(params: &Params) -> Self {
        Self {
            n_p: params.get("n_p").unwrap().clone(),
            v_pr: params.get("v_pr").unwrap().clone(),
            w_uav: params.get("w_uav").unwrap().clone(),
        }
    }

    pub fn weight(self) -> f64 {
        let mut r = 62.36;
        r *= self.n_p.powf(0.25);
        r *= f64::powf(self.v_pr * 0.001, 0.604);
        r *= self.w_uav.powf(0.1);
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self) -> f64 {
        9.90
    }
}

//------------------------------------------ anti icing

struct AntiIcing {
}

impl AntiIcing {
    pub fn new(params: &Params) -> Self {
        Self {
        }
    }

    /// - w_dg: design gross weight in lb
    pub fn weight(self, w_dg: f64) -> f64 {
        let r = 0.002 * w_dg;
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self) -> f64 {
        9.90
    }
}

//------------------------------------------ handling gear

struct HandlingGear {
}

impl HandlingGear {
    pub fn new(params: &Params) -> Self {
        Self {
        }
    }

    /// - w_dg: design gross weight in lb
    pub fn weight(self, w_dg: f64) -> f64 {
        let r = 0.0003 * w_dg;
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self) -> f64 {
        9.90
    }
}
