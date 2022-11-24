use crate::utils::point::Point;
use crate::utils::weight::Weight;
use crate::components::Component;
use crate::Params;

pub struct Fuselage {
    // weight
    k_door: Option<f64>, // 1.0 if no cargo door; 1.06 for one side cargo door; 1.12 for two side cargo doors; 1.12 for aft clamshell door; 1.25 for two side and an aft clamshell cargo doors
    k_lg: Option<f64>, // 1.12 for fuselage mounted landing gear; 1.0 otherwise
    l: Option<f64>, // Fuselage structural length (ft)
    s_f: Option<f64>, // Fuselage wetted area (ft2)
    k_ws: Option<f64>, // 0.75[(1 + 2λ)/(1 + λ)]Bw tan Λ/L
    d: Option<f64>, // Maximum fuselage diameter (ft)
    // balance
    pos_cg_f: Option<f64>, // horizontal CG position of the fuselage (given as % fuselage length and measured from the nose), 42 −45% for wing mounted engines
}

impl Component for Fuselage {
    fn new(params: &Params) -> Self {
        Self {
            // weight
            k_door: params.get("k_door").copied(),
            k_lg: params.get("k_lg").copied(),
            l: params.get("l").copied(),
            s_f: params.get("s_f").copied(),
            k_ws: params.get("k_ws").copied(),
            d: params.get("d").copied(),
            // balance
            pos_cg_f: params.get("pos_cg_f").copied(),
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            0.3280 *
            self.k_door? *
            self.k_lg? *
            f64::powf(w_dg * n_z, 0.5) *
            self.l?.powf(0.25) *
            self.s_f?.powf(0.302) *
            f64::powf(1. + self.k_ws?, 0.04) *
            f64::powf(self.l? / self.d?, 0.1)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        Some(Point::new(self.pos_cg_f? * self.l?, 0., 0.))
    }
}
