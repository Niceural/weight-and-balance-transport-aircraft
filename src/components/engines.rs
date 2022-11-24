use crate::utils::point::Point;
use crate::utils::weight::Weight;
use crate::components::Component;
use crate::Params;

#[derive(Clone, Copy)]
pub struct Engines {
    nacelle: Nacelle,
    engine_controls: EngineControls,
    engine_pneumatic_starter: EnginePneumaticStarter,
    fuel_system: FuelSystem,
}

impl Component for Engines {
    fn new(params: &Params) -> Self {
        Self {
            nacelle: Nacelle::new(params),
            engine_controls: EngineControls::new(params),
            engine_pneumatic_starter: EnginePneumaticStarter::new(params),
            fuel_system: FuelSystem::new(params),
        }
    }

    /// - n_z: ultimate load factor, 1.5x limit load factor
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(
            self.nacelle.weight(w_dg, n_z)? +
            self.engine_controls.weight(w_dg, n_z)? +
            self.engine_pneumatic_starter.weight(w_dg, n_z)? +
            self.fuel_system.weight(w_dg, n_z)?
        )
    }

    fn cg(self) -> Option<Point<f64>> {
        Some(Point::new(0., 0., 0.))
    }
}

//---------------------------------------------------- nacelle

#[derive(Copy, Clone)]
struct Nacelle {
    k_ng: Option<f64>, // 1.017 for pylon mounted nacelle, 1.0 otherwise
    n_lt: Option<f64>, // Nacelle length (ft)
    n_w: Option<f64>, // Nacelle width (ft)
    w_enc: Option<f64>, // Weight of engine and contents in lb (see pdf)
    n_en: Option<f64>, // Number of engines
    s_n: Option<f64>, // Nacelle wetted area (ft^2)
}

impl Component for Nacelle {
    fn new(params: &Params) -> Self {
        Self {
            k_ng: params.get("k_ng").copied(),
            n_lt: params.get("n_lt").copied(),
            n_w: params.get("n_w").copied(),
            w_enc: params.get("w_enc").copied(),
            n_en: params.get("n_en").copied(),
            s_n: params.get("s_n").copied(),
        }
    }

    /// - n_z: ultimate load factor, 1.5x limit load factor
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            0.6724 *
            self.k_ng? *
            f64::powf(self.n_lt?, 0.1) *
            f64::powf(self.n_w?, 0.294) *
            f64::powf(n_z, 0.119) *
            f64::powf(self.w_enc?, 0.611) *
            f64::powf(self.n_en?, 0.984) *
            f64::powf(self.s_n?, 0.224)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        Some(Point::new(0., 0., 0.))
    }
}

//---------------------------------------------------- engine controls

#[derive(Copy, Clone)]
struct EngineControls {
    l_ec: Option<f64>, // Engine controls routing distance; engine to cockpit - total if multiengine (ft)
    n_en: Option<f64>, // Number of engines
    cg: Option<Point<f64>>,
}

impl Component for EngineControls {
    fn new(params: &Params) -> Self {
        Self {
            l_ec: params.get("l_ec").copied(),
            n_en: params.get("n_en").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_engine_controls").copied(),
                params.get("y_cg_engine_controls").copied(),
                params.get("z_cg_engine_controls").copied(),
            )),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(5. * self.n_en? + 0.8 * self.l_ec?))
    }

    fn cg(self) -> Option<Point<f64>> {
        Some(Point::new(0., 0., 0.))
    }
}

//---------------------------------------------------- engine pneumatic starter

#[derive(Copy, Clone)]
struct EnginePneumaticStarter {
    n_en: Option<f64>, // Number of engines
    w_en: Option<f64>, // Engine weight (lb)
    cg: Option<Point<f64>>,
}

impl Component for EnginePneumaticStarter {
    fn new(params: &Params) -> Self {
        Self {
            n_en: params.get("n_en").copied(),
            w_en: params.get("w_en").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_engine_pneumatic_starter").copied(),
                params.get("y_cg_engine_pneumatic_starter").copied(),
                params.get("z_cg_engine_pneumatic_starter").copied(),
            )),
        }
    }
    
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(49.19 * f64::powf(self.n_en? * self.w_en? * 1e-3, 0.541)))
    }

    fn cg(self) -> Option<Point<f64>> {
        Some(Point::new(0., 0., 0.))
    }
}

//---------------------------------------------------- fuel system

#[derive(Copy, Clone)]
struct FuelSystem {
    v_t: Option<f64>, // Total volume of fuel tanks (gal)
    n_t: Option<f64>, // Total number of fuel tanks
    v_p: Option<f64>, // Self sealing tank volume (gal)
    v_i: Option<f64>, // Integral fuel tank volume (gal)
    cg: Option<Point<f64>>,
}

impl Component for FuelSystem {
    fn new(params: &Params) -> Self {
        Self {
            v_t: params.get("v_t").copied(),
            n_t: params.get("n_t").copied(),
            v_p: params.get("v_p").copied(),
            v_i: params.get("v_i").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_fuel_system").copied(),
                params.get("y_cg_fuel_system").copied(),
                params.get("z_cg_fuel_system").copied(),
            )),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            2.405 *
            self.v_t?.powf(0.606) *
            self.n_t?.powf(0.5) *
            1. + self.v_p? / self.v_t? /
            (1. + self.v_i? / self.v_t?)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}
