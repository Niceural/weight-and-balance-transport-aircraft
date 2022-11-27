use crate::utils::point::Point;
use crate::utils::weight::Weight;
use crate::components::Component;
use crate::Params;

#[derive(Clone, Copy)]
pub struct Systems {
    flight_controls: FlightControls,
    installed_apu: InstalledApu,
    anti_icing: AntiIcing,
    handling_gear: HandlingGear,
}

impl Component for Systems {
    fn new(params: &Params) -> Self {
        Self {
            flight_controls: FlightControls::new(params),
            installed_apu: InstalledApu::new(params),
            instruments: Instruments::new(params),
            hydrolic_system: HydrolicSystem::new(params),
            electrical_systems: ElectricalSystems::new(params),
            avionics: Avionics::new(params),
            furnishing: Furnishing::new(params),
            air_conditioning: AirConditioning::new(params),
            anti_icing: AntiIcing::new(params),
            handling_gear: HandlingGear::new(params),
        }
    }

    /// - w_dg: design gross weight in lb
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(
            self.flight_controls.weight(w_dg, n_z)? +
            self.installed_apu.weight(w_dg, n_z)? +
            self.instruments.weight(w_dg, n_z)? +
            self.hydrolic_system.weight(w_dg, n_z)? +
            self.electrical_systems.weight(w_dg, n_z)? +
            self.avionics.weight(w_dg, n_z)? +
            self.furnishing.weight(w_dg, n_z)? +
            self.air_conditioning.weight(w_dg, n_z)? +
            self.anti_icing.weight(w_dg, n_z)? +
            self.handling_gear.weight(w_dg, n_z)?
        )
    }

    fn cg(self) -> Option<Point<f64>> {
        Some(
            self.flight_controls.cg()? +
            self.installed_apu.cg()? +
            self.instruments.cg()? +
            self.hydrolic_system.cg()? +
            self.electrical_systems.cg()? +
            self.avionics.cg()? +
            self.furnishing.cg()? +
            self.air_conditioning.cg()? +
            self.anti_icing.cg()? +
            self.handling_gear.cg()?
        )
    }
}

