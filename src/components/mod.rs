pub mod wings;
pub mod tailplane;
pub mod systems;
pub mod fuselage;
pub mod landing_gear;
pub mod engines;
use crate::components::{
    wings::Wings,
    tailplane::Tailplane,
    engines::Engines,
    fuselage::Fuselage,
    landing_gear::LandingGear,
};
use crate::Params;

pub struct Aircraft {
    wings: Wings,
    tailplane: Tailplane,
    engines: Engines,
    fuselage: Fuselage,
    landing_gear: LandingGear,
}

impl Aircraft {
    pub fn new(params: &Params) -> Self {
        let wings = Wings::new(params);
        let tailplane = Tailplane::new(params);
        let engines = Engines::new(params);
        let fuselage = Fuselage::new(params);
        let landing_gear = LandingGear::new(params);

        Self {
            wings,
            tailplane,
            engines,
            fuselage,
            landing_gear,
        }
    }

    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        let mut r = self.wings.weight(w_dg, n_z);
        r += self.tailplane.weight(w_dg, n_z);
        r += self.engines.weight(n_z);
        r += self.fuselage.weight(w_dg, n_z);
        r += self.landing_gear.weight();
        if r < 0. { panic!("negative weight"); }
        r
    }
}
