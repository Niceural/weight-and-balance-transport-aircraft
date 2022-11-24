use crate::utils::point::Point;
use crate::utils::weight::Weight;
use crate::components::Component;
use crate::Params;

#[derive(Copy, Clone)]
pub struct LandingGear {
    main: MainLandingGear,
    nose: NoseLandingGear,
}

impl Component for LandingGear {
    fn new(params: &Params) -> Self {
        Self {
            main: MainLandingGear::new(params),
            nose: NoseLandingGear::new(params),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(
            self.main.weight(w_dg, n_z)? +
            self.nose.weight(w_dg, n_z)?
        )
    }

    fn cg(self) -> Option<Point<f64>> {
        Some(Point::new(0., 0., 0.))
    }
}

#[derive(Copy, Clone)]
struct MainLandingGear{
    w_l: Option<f64>, // Landing design gross weight (lb)
    n_l: Option<f64>, // Ultimate landing gear load factor. 1.5 × N_gear
    k_mp: Option<f64>, // 1.126 for kneeling main gear; 1.0 otherwise
    l_m: Option<f64>, // Main landing gear length (inches)
    n_mw: Option<f64>, // Number of main wheels
    v_s: Option<f64>, // Landing stall speed (ft/s)
    n_mss: Option<f64>, // Number of main gear shock struts
}


impl Component for MainLandingGear {
    fn new(params: &Params) -> Self {
        Self {
            w_l: params.get("w_l").copied(),
            n_l: params.get("n_l").copied(),
            k_mp: params.get("k_mp").copied(),
            l_m: params.get("l_m").copied(),
            n_mw: params.get("n_mw").copied(),
            v_s: params.get("v_s").copied(),
            n_mss: params.get("n_mss").copied(),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            0.0106 *
            self.k_mp? *
            self.w_l?.powf(0.888) *
            self.n_l?.powf(0.25) *
            self.l_m?.powf(0.4) *
            self.n_mw?.powf(0.321) *
            self.v_s?.powf(0.1) /
            self.n_mss?.powf(0.5)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        Some(Point::new(0., 0., 0.))
    }
}

#[derive(Copy, Clone)]
struct NoseLandingGear{
    w_l: Option<f64>, // Landing design gross weight (lb)
    n_l: Option<f64>, // Ultimate landing gear load factor. 1.5 × N_gear
    k_np: Option<f64>, // 1.15 for kneeling nose-gear; 1.0 otherwise
    l_n: Option<f64>, // Nose landing gear length (inches)
    n_nw: Option<f64>, // Number of nose wheels
}

impl Component for NoseLandingGear {
    fn new(params: &Params) -> Self {
        Self {
            w_l: params.get("w_l").copied(),
            n_l: params.get("n_l").copied(),
            k_np: params.get("k_np").copied(),
            l_n: params.get("l_n").copied(),
            n_nw: params.get("n_nw").copied(),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            0.032 *
            self.k_np? *
            self.w_l?.powf(0.646) *
            self.n_l?.powf(0.2) *
            self.l_n?.powf(0.5) *
            self.n_nw?.powf(0.45)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        Some(Point::new(0., 0., 0.))
    }
}
