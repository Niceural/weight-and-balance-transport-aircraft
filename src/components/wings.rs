use crate::utils::point::Point;
use crate::utils::weight::Weight;
use crate::Params;

pub struct Wings {
    wings_structure: WingsStructure,
    nacelle: Nacelle,
    engine_controls: EngineControls,
    fuel_system: FuelSystem,
    engine_pneumatic_starter: EnginePneumaticStarter,
}

impl Wings {
    pub fn new(params: &Params) -> Self {
        Self {
            wings_structure: WingsStructure::new(params),
            nacelle: Nacelle::new(params),
            engine_controls: EngineControls::new(params),
            fuel_system: FuelSystem::new(params),
            engine_pneumatic_starter: EnginePneumaticStarter::new(params),
        }
    }

    pub fn weight(&self, w_dg: f64) -> Weight {
        self.wings_structure.weight(w_dg) +
        self.nacelle.weight() +
        self.engine_controls.weight() +
        self.fuel_system.weight() +
        self.engine_pneumatic_starter.weight()
    }

    /// wings_root_pos: position of the root of the wing
    pub fn pos_times_weight(&self, w_dg: f64, wings_root_pos: Point<f64>) -> Point<f64> {
        (wings_root_pos + self.wings_structure.cg()) * self.wings_structure.weight(w_dg).get_val() +
        (wings_root_pos + self.nacelle.cg()) * self.nacelle.weight().get_val() +
        (wings_root_pos + self.engine_controls.cg()) * self.engine_controls.weight().get_val() +
        (wings_root_pos + self.fuel_system.cg()) * self.fuel_system.weight().get_val() +
        (wings_root_pos + self.engine_pneumatic_starter.cg()) * self.engine_pneumatic_starter.weight().get_val()
    }
}

//------------------------------------------- wings structure

struct WingsStructure {
    // weights
    n_z: f64,
    s_w: f64,            // reference wing area in ft^2
    ar: f64,             // wing aspect ratio
    lambda: f64,         // wing taper ratio
    s_csw: f64,          // area of wing mounted control surfaces in ft^2
    t_c_ratio_root: f64, // wing root thickness to chord ratio
    sweep: f64,        // wing quarter chord sweep in rad
    // balance
    delta_fs_as: f64, // distance between forward spar and aft spar at centerline in ft
    x_fs: f64,        // centerline forward spar position from root in ft
    sweep_fs: f64,    // forward spar sweep in radians
    sweep_as: f64,    // aft spar sweep in radians
    dihedral: f64,    // wing quarter chord dihedral in radians
    wing_span: f64,   // wing span in ft
}

impl WingsStructure {
    pub fn new(params: &Params) -> Self {
        Self {
            // weights
            n_z: params.get("n_z").expect("missing n_z").clone(),
            s_w: params.get("s_w").expect("missing s_w").clone(),
            ar: params.get("ar").expect("missing ar").clone(),
            lambda: params.get("lambda").expect("missing lambda").clone(),
            s_csw: params.get("s_csw").expect("missing s_csw").clone(),
            t_c_ratio_root: params.get("t_c_ratio_root").expect("missing t_c_ratio_root").clone(),
            sweep: params.get("sweep").expect("missing sweep").clone(),
            // balance
            delta_fs_as: params.get("delta_fs_as").expect("missing delta_fs_as").clone(),
            x_fs: params.get("x_fs").expect("missing x_fs").clone(),
            sweep_fs: params.get("sweep_fs").expect("missing sweep_fs").clone(),
            sweep_as: params.get("sweep_as").expect("missing sweep_as").clone(),
            dihedral: params.get("dihedral").expect("missing dihedral").clone(),
            wing_span: params.get("wing_span").expect("missing wing_span").clone(),
        }
    }

    /// - w_dg: design gross weight in lb
    pub fn weight(&self, w_dg: f64) -> Weight {
        Weight::new(
            // numerator
            0.0051 *
            f64::powf(w_dg * self.n_z, 0.557) *
            self.s_w.powf(0.649) *
            self.ar.powf(0.5) *
            f64::powf(1.0 + self.lambda, 0.1) *
            self.s_csw.powf(0.1) /
            // denominator
            self.sweep.cos() /
            self.t_c_ratio_root.powf(0.4)
        )
    }

    /// - pos_w: position of quarter chord point in ft
    pub fn cg(&self) -> Point<f64> {
        // 35% semi span from centerline
        let y = 0.35 * self.wing_span * 0.5;
        let z = y * self.dihedral.tan();
        // opposite sides of triangles
        let opposite_fs = y * self.sweep_fs.tan();
        let opposite_as = y * self.sweep_as.tan();
        let difference_07 = (self.delta_fs_as + opposite_as - opposite_fs) * 0.7;
        let x = difference_07 + opposite_fs + self.x_fs;
        Point::new(x, 0., z)
    }
}

//---------------------------------------------------- nacelle

struct Nacelle {
    n_z: f64,
    k_ng: f64, // 1.017 for pylon mounted nacelle, 1.0 otherwise
    n_lt: f64, // Nacelle length (ft)
    n_w: f64, // Nacelle width (ft)
    w_enc: f64, // Weight of engine and contents in lb (see pdf)
    n_en: f64, // Number of engines
    s_n: f64, // Nacelle wetted area (ft^2)
    cg: Point<f64>,
}

impl Nacelle {
    pub fn new(params: &Params) -> Self {
        Self {
            n_z: params.get("n_z").expect("missing n_z").clone(),
            k_ng: params.get("k_ng").expect("missing k_ng").clone(),
            n_lt: params.get("n_lt").expect("missing n_lt").clone(),
            n_w: params.get("n_w").expect("missing n_w").clone(),
            w_enc: params.get("w_enc").expect("missing w_enc").clone(),
            n_en: params.get("n_en").expect("missing n_en").clone(),
            s_n: params.get("s_n").expect("missing s_n").clone(),
            cg: Point::new(
                params.get("x_cg_nacelle").expect("missing x_cg_nacelle").clone(),
                params.get("y_cg_nacelle").expect("missing y_cg_nacelle").clone(),
                params.get("z_cg_nacelle").expect("missing z_cg_nacelle").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(0.6724 *
            self.k_ng *
            f64::powf(self.n_lt, 0.1) *
            f64::powf(self.n_w, 0.294) *
            f64::powf(self.n_z, 0.119) *
            f64::powf(self.w_enc, 0.611) *
            f64::powf(self.n_en, 0.984) *
            f64::powf(self.s_n, 0.224))
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//---------------------------------------------------- engine controls

struct EngineControls {
    l_ec: f64, // Engine controls routing distance; engine to cockpit - total if multiengine (ft)
    n_en: f64, // Number of engines
    cg: Point<f64>,
}

impl EngineControls {
    pub fn new(params: &Params) -> Self {
        Self {
            l_ec: params.get("l_ec").expect("missing l_ec").clone(),
            n_en: params.get("n_en").expect("missing n_en").clone(),
            cg: Point::new(
                params.get("x_cg_engine_controls").expect("missing x_cg_engine_controls").clone(),
                params.get("y_cg_engine_controls").expect("missing y_cg_engine_controls").clone(),
                params.get("z_cg_engine_controls").expect("missing z_cg_engine_controls").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(5. * self.n_en + 0.8 * self.l_ec)
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//---------------------------------------------------- fuel system

struct FuelSystem {
    v_t: f64, // Total volume of fuel tanks (gal)
    n_t: f64, // Total number of fuel tanks
    v_p: f64, // Self sealing tank volume (gal)
    v_i: f64, // Integral fuel tank volume (gal)
    cg: Point<f64>,
}

impl FuelSystem {
    pub fn new(params: &Params) -> Self {
        Self {
            v_t: params.get("v_t").expect("missing v_t").clone(),
            n_t: params.get("n_t").expect("missing n_t").clone(),
            v_p: params.get("v_p").expect("missing v_p").clone(),
            v_i: params.get("v_i").expect("missing v_i").clone(),
            cg: Point::new(
                params.get("x_cg_fuel_system").expect("missing x_cg_fuel_system").clone(),
                params.get("y_cg_fuel_system").expect("missing y_cg_fuel_system").clone(),
                params.get("z_cg_fuel_system").expect("missing z_cg_fuel_system").clone(),
            ),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(
            2.405 *
            self.v_t.powf(0.606) *
            self.n_t.powf(0.5) *
            1. + self.v_p / self.v_t /
            (1. + self.v_i / self.v_t)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

//---------------------------------------------------- engine pneumatic starter

struct EnginePneumaticStarter {
    n_en: f64, // Number of engines
    w_en: f64, // Engine weight (lb)
    cg: Point<f64>,
}

impl EnginePneumaticStarter {
    pub fn new(params: &Params) -> Self {
        Self {
            n_en: params.get("n_en").expect("missing n_en").clone(),
            w_en: params.get("w_en").expect("missing w_en").clone(),
            cg: Point::new(
                params.get("x_cg_engine_pneumatic_starter").expect("missing x_cg_engine_pneumatic_starter").clone(),
                params.get("y_cg_engine_pneumatic_starter").expect("missing y_cg_engine_pneumatic_starter").clone(),
                params.get("z_cg_engine_pneumatic_starter").expect("missing z_cg_engine_pneumatic_starter").clone(),
            ),
        }
    }
    
    pub fn weight(&self) -> Weight {
        Weight::new(49.19 * f64::powf(self.n_en * self.w_en * 1e-3, 0.541))
    }

    pub fn cg(&self) -> Point<f64> {
        self.cg
    }
}

