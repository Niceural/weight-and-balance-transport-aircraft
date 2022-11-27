use crate::utils::point::Point;
use crate::utils::weight::Weight;
use crate::Params;

pub struct Fuselage {
    fuselage_structure: FuselageStructure,
    hydrolic_system: HydrolicSystem,
    main_landing_gear: MainLandingGear,
    nose_landing_gear: NoseLandingGear,
    furnishing: Furnishing,
    air_conditioning: AirConditioning,
    electrical_systems: ElectricalSystems,
    instruments: Instruments,
    avionics: Avionics,
    flight_controls: FlightControls,
    installed_apu: InstalledApu,
    anti_icing: AntiIcing,
    handling_gear: HandlingGear,
}

impl Fuselage {
    pub fn new(params: &Params) -> Self {
        Self {
            fuselage_structure: FuselageStructure::new(params),
            hydrolic_system: HydrolicSystem::new(params),
            main_landing_gear: MainLandingGear::new(params),
            nose_landing_gear: NoseLandingGear::new(params),
            furnishing: Furnishing::new(params),
            air_conditioning: AirConditioning::new(params),
            electrical_systems: ElectricalSystems::new(params),
            instruments: Instruments::new(params),
            avionics: Avionics::new(params),
            flight_controls: FlightControls::new(params),
            installed_apu: InstalledApu::new(params),
            anti_icing: AntiIcing::new(params),
            handling_gear: HandlingGear::new(params),
        }
    }

    pub fn weight(&self, w_dg: f64) -> Weight {
        self.fuselage_structure.weight(w_dg) +
        self.hydrolic_system.weight() +
        self.main_landing_gear.weight() +
        self.nose_landing_gear.weight() +
        self.furnishing.weight() +
        self.air_conditioning.weight() +
        self.electrical_systems.weight() +
        self.instruments.weight() +
        self.avionics.weight() +
        self.flight_controls.weight(w_dg) +
        self.installed_apu.weight(w_dg) +
        self.anti_icing.weight(w_dg) +
        self.handling_gear.weight(w_dg) 
    }

    pub fn pos_times_weight(&self, w_dg: f64) -> Point<f64> {
        self.fuselage_structure.cg() * self.fuselage_structure.weight(w_dg).get_val() +
        self.hydrolic_system.cg() * self.hydrolic_system.weight().get_val() +
        self.main_landing_gear.cg() * self.main_landing_gear.weight().get_val() +
        self.nose_landing_gear.cg() * self.nose_landing_gear.weight().get_val() +
        self.furnishing.cg() * self.furnishing.weight().get_val() +
        self.air_conditioning.cg() * self.air_conditioning.weight().get_val() +
        self.electrical_systems.cg() * self.electrical_systems.weight().get_val() +
        self.instruments.cg() * self.instruments.weight().get_val() +
        self.avionics.cg() * self.avionics.weight().get_val() +
        self.flight_controls.cg() * self.flight_controls.weight(w_dg).get_val() +
        self.installed_apu.cg() * self.installed_apu.weight(w_dg).get_val() +
        self.anti_icing.cg() * self.anti_icing.weight(w_dg).get_val() +
        self.handling_gear.cg() * self.handling_gear.weight(w_dg).get_val()
    }
}

//------------------------------------------ fuselage structure

struct FuselageStructure {
    // weight
    n_z: f64,
    k_door: f64, // 1.0 if no cargo door; 1.06 for one side cargo door; 1.12 for two side cargo doors; 1.12 for aft clamshell door; 1.25 for two side and an aft clamshell cargo doors
    k_lg: f64, // 1.12 for fuselage mounted landing gear; 1.0 otherwise
    l: f64, // Fuselage structural length (ft)
    s_f: f64, // Fuselage wetted area (ft2)
    k_ws: f64, // 0.75[(1 + 2λ)/(1 + λ)]Bw tan Λ/L
    d: f64, // Maximum fuselage diameter (ft)
    // balance
    pos_cg_f: f64, // horizontal CG position of the fuselage (given as % fuselage length and measured from the nose), 42 −45% for wing mounted engines
}

impl FuselageStructure {
    pub fn new(params: &Params) -> Self {
        Self {
            // weight
            n_z: params.get("n_z").expect("missing  n_z").clone(),
            k_door: params.get("k_door").expect("missing  k_door").clone(),
            k_lg: params.get("k_lg").expect("missing  k_lg").clone(),
            l: params.get("l").expect("missing  l").clone(),
            s_f: params.get("s_f").expect("missing  s_f").clone(),
            k_ws: params.get("k_ws").expect("missing  k_ws").clone(),
            d: params.get("d").expect("missing  d").clone(),
            // balance
            pos_cg_f: params.get("pos_cg_f").expect("missing  pos_cg_f").clone(),
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(&self, w_dg: f64) -> Weight {
        Weight::new(
            0.3280 *
            self.k_door *
            self.k_lg *
            f64::powf(w_dg * self.n_z, 0.5) *
            self.l.powf(0.25) *
            self.s_f.powf(0.302) *
            f64::powf(1. + self.k_ws, 0.04) *
            f64::powf(self.l / self.d, 0.1)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        Point::new(self.pos_cg_f * self.l, 0., 0.)
    }
}

//------------------------------------------ hydrolic system

struct HydrolicSystem {
    n_f: f64,
    l_f: f64,
    b_w: f64,
    cg: Point<f64>,
}

impl HydrolicSystem {
    pub fn new(params: &Params) -> Self {
        Self {
            n_f: params.get("n_f").expect("missing n_f").clone(),
            l_f: params.get("l_f").expect("missing l_f").clone(),
            b_w: params.get("b_w").expect("missing b_w").clone(),
            cg: Point::new(
                params.get("x_cg_hydrolic_system").expect("missing x_cg_hydrolic_system").clone(),
                params.get("y_cg_hydrolic_system").expect("missing y_cg_hydrolic_system").clone(),
                params.get("z_cg_hydrolic_system").expect("missing z_cg_hydrolic_system").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(
            0.2673 *
            self.n_f *
            f64::powf(self.l_f + self.b_w, 0.937)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//------------------------------------------ main landing gear

pub struct MainLandingGear {
    w_l: f64, // Landing design gross weight (lb)
    n_l: f64, // Ultimate landing gear load factor. 1.5 × N_gear
    k_mp: f64, // 1.126 for kneeling main gear; 1.0 otherwise
    l_m: f64, // Main landing gear length (inches)
    n_mw: f64, // Number of main wheels
    v_s: f64, // Landing stall speed (ft/s)
    n_mss: f64, // Number of main gear shock struts
    cg: Point<f64>, // center of gravity of main landing gear
}


impl MainLandingGear {
    pub fn new(params: &Params) -> Self {
        Self {
            w_l: params.get("w_l").expect("missing w_l").clone(),
            n_l: params.get("n_l").expect("missing n_l").clone(),
            k_mp: params.get("k_mp").expect("missing k_mp").clone(),
            l_m: params.get("l_m").expect("missing l_m").clone(),
            n_mw: params.get("n_mw").expect("missing n_mw").clone(),
            v_s: params.get("v_s").expect("missing v_s").clone(),
            n_mss: params.get("n_mss").expect("missing n_mss").clone(),
            cg: Point::new(
                params.get("x_cg_main_landing_gear").expect("missing x_cg_main_landing_gear").clone(),
                params.get("y_cg_main_landing_gear").expect("missing y_cg_main_landing_gear").clone(),
                params.get("z_cg_main_landing_gear").expect("missing z_cg_main_landing_gear").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(
            0.0106 *
            self.k_mp *
            self.w_l.powf(0.888) *
            self.n_l.powf(0.25) *
            self.l_m.powf(0.4) *
            self.n_mw.powf(0.321) *
            self.v_s.powf(0.1) /
            self.n_mss.powf(0.5)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//------------------------------------------ nose landing gear

pub struct NoseLandingGear {
    w_l: f64, // Landing design gross weight (lb)
    n_l: f64, // Ultimate landing gear load factor. 1.5 × N_gear
    k_np: f64, // 1.15 for kneeling nose-gear; 1.0 otherwise
    l_n: f64, // Nose landing gear length (inches)
    n_nw: f64, // Number of nose wheels
    cg: Point<f64>, // center of gravity of main landing gear
}

impl NoseLandingGear {
    pub fn new(params: &Params) -> Self {
        Self {
            w_l: params.get("w_l").expect("missing w_l").clone(),
            n_l: params.get("n_l").expect("missing n_l").clone(),
            k_np: params.get("k_np").expect("missing k_np").clone(),
            l_n: params.get("l_n").expect("missing l_n").clone(),
            n_nw: params.get("n_nw").expect("missing n_nw").clone(),
            cg: Point::new(
                params.get("x_cg_nose_landing_gear").expect("missing x_cg_nose_landing_gear").clone(),
                params.get("y_cg_nose_landing_gear").expect("missing y_cg_nose_landing_gear").clone(),
                params.get("z_cg_nose_landing_gear").expect("missing z_cg_nose_landing_gear").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(
            0.032 *
            self.k_np *
            self.w_l.powf(0.646) *
            self.n_l.powf(0.2) *
            self.l_n.powf(0.5) *
            self.n_nw.powf(0.45)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
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
    cg: Point<f64>,
}

impl Furnishing {
    pub fn new(params: &Params) -> Self {
        Self {
            n_c: params.get("n_c").expect("missing n_c").clone(),
            w_c: params.get("w_c").expect("missing w_c").clone(),
            s_f: params.get("s_f").expect("missing s_f").clone(),
            n_seat: params.get("n_seat").expect("missing n_seat").clone(),
            w_seat: params.get("w_seat").expect("missing w_seat").clone(),
            k_lav: params.get("k_lav").expect("missing k_lav").clone(),
            n_p: params.get("n_p").expect("missing n_p").clone(),
            k_buf: params.get("k_buf").expect("missing k_buf").clone(),
            cg: Point::new(
                params.get("x_cg_furnishing").expect("missing x_cg_furnishing").clone(),
                params.get("y_cg_furnishing").expect("missing y_cg_furnishing").clone(),
                params.get("z_cg_furnishing").expect("missing z_cg_furnishing").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(
            0.0577 * self.n_c.powf(0.1) *
            self.w_c.powf(0.393) * self.s_f.powf(0.75) +
            self.n_seat * self.w_seat +
            self.k_lav * self.n_p.powf(1.33) +
            self.k_buf * self.n_p.powf(1.12)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//------------------------------------------ air conditioning

struct AirConditioning {
    n_p: f64,
    v_pr: f64,
    w_uav: f64,
    cg: Point<f64>,
}

impl AirConditioning {
    pub fn new(params: &Params) -> Self {
        Self {
            n_p: params.get("n_p").expect("missing n_p").clone(),
            v_pr: params.get("v_pr").expect("missing v_pr").clone(),
            w_uav: params.get("w_uav").expect("missing w_uav").clone(),
            cg: Point::new(
                params.get("x_cg_air_conditioning").expect("missing x_cg_air_conditioning").clone(),
                params.get("y_cg_air_conditioning").expect("missing y_cg_air_conditioning").clone(),
                params.get("z_cg_air_conditioning").expect("missing z_cg_air_conditioning").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(
            62.36 *
            self.n_p.powf(0.25) *
            f64::powf(self.v_pr * 0.001, 0.604) *
            self.w_uav.powf(0.1)
        )
    }

    fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//------------------------------------------ electrical systems

// #[derive(Clone, Copy)]
struct ElectricalSystems {
    r_kva: f64,
    l_a: f64,
    n_gen: f64,
    cg: Point<f64>,
}

impl ElectricalSystems {
    pub fn new(params: &Params) -> Self {
        Self {
            r_kva: params.get("r_kva").expect("missing r_kva").clone(),
            l_a: params.get("l_a").expect("missing l_a").clone(),
            n_gen: params.get("n_gen").expect("missing n_gen").clone(),
            cg: Point::new(
                params.get("x_cg_electrical_systems").expect("missing x_cg_electrical_systems").clone(),
                params.get("y_cg_electrical_systems").expect("missing y_cg_electrical_systems").clone(),
                params.get("z_cg_electrical_systems").expect("missing z_cg_electrical_systems").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(
            7.291 *
            self.r_kva.powf(0.782) *
            self.l_a.powf(0.346) *
            self.n_gen.powf(0.1)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//------------------------------------------ instruments

struct Instruments {
    k_r: f64,
    n_c: f64,
    n_en: f64,
    l_f: f64,
    b_w: f64,
    cg: Point<f64>,
}

impl Instruments {
    pub fn new(params: &Params) -> Self {
        Self {
            k_r: params.get("k_r").expect("missing k_r").clone(),
            n_c: params.get("n_c").expect("missing n_c").clone(),
            n_en: params.get("n_en").expect("missing n_en").clone(),
            l_f: params.get("l_f").expect("missing l_f").clone(),
            b_w: params.get("b_w").expect("missing b_w").clone(),
            cg: Point::new(
                params.get("x_cg_instruments").expect("missing x_cg_instruments").clone(),
                params.get("y_cg_instruments").expect("missing y_cg_instruments").clone(),
                params.get("z_cg_instruments").expect("missing z_cg_instruments").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(
            4.509 *
            self.k_r *
            self.n_c.powf(0.541) *
            self.n_en *
            f64::powf(self.l_f + self.b_w, 0.5)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//------------------------------------------ avionics

struct Avionics {
    w_uav: f64,
    cg: Point<f64>,
}

impl Avionics {
    pub fn new(params: &Params) -> Self {
        Self {
            w_uav: params.get("w_uav").expect("missing w_uav").clone(),
            cg: Point::new(
                params.get("x_cg_avionics").expect("missing w_uav").clone(),
                params.get("y_cg_avionics").expect("missing ").clone(),
                params.get("z_cg_avionics").expect("missing ").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(1.73 * self.w_uav.powf(0.983))
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//------------------------------------------ flight controls

struct FlightControls {
    n_f: f64,
    s_cs: f64,
    i_y: f64,
    n_m: f64,
    n_z: f64,
    // balance
    cg: Point<f64>,
}

impl FlightControls {
    pub fn new(params: &Params) -> Self {
        Self {
            n_f: params.get("n_f").expect("missing n_f").clone(),
            s_cs: params.get("s_cs").expect("missing s_cs").clone(),
            i_y: params.get("i_y").expect("missing i_y").clone(),
            n_m: params.get("n_m").expect("missing n_m").clone(),
            n_z: params.get("n_z").expect("missing n_z").clone(),
            cg: Point::new(
                params.get("x_cg_flight_controls").expect("missing x_cg_flight_controls").clone(),
                params.get("y_cg_flight_controls").expect("missing y_cg_flight_controls").clone(),
                params.get("z_cg_flight_controls").expect("missing z_cg_flight_controls").clone(),
            ),
        }
    }

    pub fn weight(&self, w_dg: f64) -> Weight {
        Weight::new(
            // numerator
            145.9 *
            self.n_f.powf(0.554) *
            self.s_cs.powf(0.2) *
            f64::powf(self.i_y * 1e-6, 0.07) /
            // denominator
            (1. + self.n_m / self.n_f)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//------------------------------------------ installed apu

struct InstalledApu {
    w_apu: f64,
    n_z: f64,
    cg: Point<f64>,
}

impl InstalledApu {
    pub fn new(params: &Params) -> Self {
        Self {
            w_apu: params.get("w_apu").expect("missing w_apu").clone(),
            n_z: params.get("n_z").expect("missing n_z").clone(),
            cg: Point::new(
                params.get("x_cg_installed_apu").expect("missing x_cg_installed_apu").clone(),
                params.get("y_cg_installed_apu").expect("missing y_cg_installed_apu").clone(),
                params.get("z_cg_installed_apu").expect("missing z_cg_installed_apu").clone(),
            ),
        }
    }

    pub fn weight(&self, w_dg: f64) -> Weight {
        Weight::new(2.2 * self.w_apu)
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//------------------------------------------ anti icing

struct AntiIcing {
    n_z: f64,
    cg: Point<f64>,
}

impl AntiIcing {
    pub fn new(params: &Params) -> Self {
        Self {
            n_z: params.get("n_z").expect("missing n_z").clone(),
            cg: Point::new(
                params.get("x_cg_anti_icing").expect("missing x_cg_anti_icing").clone(),
                params.get("y_cg_anti_icing").expect("missing y_cg_anti_icing").clone(),
                params.get("z_cg_anti_icing").expect("missing z_cg_anti_icing").clone(),
            ),
        }
    }

    /// - w_dg: design gross weight in lb
    pub fn weight(&self, w_dg: f64) -> Weight {
        Weight::new(0.002 * w_dg)
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//------------------------------------------ handling gear

struct HandlingGear {
    n_z: f64,
    cg: Point<f64>,
}

impl HandlingGear {
    pub fn new(params: &Params) -> Self {
        Self {
            n_z: params.get("n_z").expect("missing n_z").clone(),
            cg: Point::new(
                params.get("x_cg_handling_gear").expect("missing x_cg_handling_gear").clone(),
                params.get("y_cg_handling_gear").expect("missing y_cg_handling_gear").clone(),
                params.get("z_cg_handling_gear").expect("missing z_cg_handling_gear").clone(),
            ),
        }
    }

    /// - w_dg: design gross weight in lb
    pub fn weight(&self, w_dg: f64) -> Weight {
        Weight::new(0.0003 * w_dg)
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}
