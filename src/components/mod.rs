pub mod wings;
pub mod tailplane;
pub mod varying;
pub mod fuselage;
use crate::Params;
use crate::utils::point::Point;
use crate::utils::weight::Weight;

pub trait Component {
    fn new(params: &Params) -> Self;

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight>;

    fn cg(self) -> Option<Point<f64>>;
}

/*

use crate::components::{
    wings::Wings,
    tailplane::Tailplane,
    engines::Engines,
    fuselage::Fuselage,
    landing_gear::LandingGear,
    systems::Systems,
};
pub struct Aircraft {
    wings: Wings,
    tailplane: Tailplane,
    engines: Engines,
    fuselage: Fuselage,
    landing_gear: LandingGear,
    systems: Systems
}

impl Aircraft {
    pub fn new(params: &Params) -> Self {
        let wings = Wings::new(params);
        let tailplane = Tailplane::new(params);
        let engines = Engines::new(params);
        let fuselage = Fuselage::new(params);
        let landing_gear = LandingGear::new(params);
        let systems = Systems::new(params);

        Self {
            wings,
            tailplane,
            engines,
            fuselage,
            landing_gear,
            systems,
        }
    }

    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        let mut r = self.wings.weight(w_dg, n_z);
        r += self.tailplane.weight(w_dg, n_z);
        r += self.engines.weight(n_z);
        r += self.fuselage.weight(w_dg, n_z);
        r += self.landing_gear.weight();
        r += self.systems.weight(w_dg);
        if r < 0. { panic!("negative weight"); }
        r
    }
}
*/
