use crate::utils::point::Point;
use crate::utils::weight::Weight;
use crate::components::Component;
use crate::Params;

#[derive(Clone, Copy)]
pub struct Systems {
    flight_controls: FlightControls,
    installed_apu: InstalledApu,
    instruments: Instruments,
    hydrolic_system: HydrolicSystem,
    electrical_systems: ElectricalSystems,
    avionics: Avionics,
    furnishing: Furnishing,
    air_conditioning: AirConditioning,
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

//------------------------------------------ flight controls

#[derive(Clone, Copy)]
struct FlightControls {
    n_f: Option<f64>,
    s_cs: Option<f64>,
    i_y: Option<f64>,
    n_m: Option<f64>,
    // balance
    cg: Option<Point<f64>>,
}

impl Component for FlightControls {
    fn new(params: &Params) -> Self {
        Self {
            n_f: params.get("n_f").copied(),
            s_cs: params.get("s_cs").copied(),
            i_y: params.get("i_y").copied(),
            n_m: params.get("n_m").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_flight_controls").copied(),
                params.get("y_cg_flight_controls").copied(),
                params.get("z_cg_flight_controls").copied(),
            )),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            // numerator
            145.9 *
            self.n_f?.powf(0.554) *
            self.s_cs?.powf(0.2) *
            f64::powf(self.i_y? * 1e-6, 0.07) /
            // denominator
            (1. + self.n_m? / self.n_f?)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}

//------------------------------------------ installed apu

#[derive(Clone, Copy)]
struct InstalledApu {
    w_apu: Option<f64>,
    cg: Option<Point<f64>>,
}

impl Component for InstalledApu {
    fn new(params: &Params) -> Self {
        Self {
            w_apu: params.get("w_apu").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_installed_apu").copied(),
                params.get("y_cg_installed_apu").copied(),
                params.get("z_cg_installed_apu").copied(),
            )),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(2.2 * self.w_apu?))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}

//------------------------------------------ instruments

#[derive(Clone, Copy)]
struct Instruments {
    k_r: Option<f64>,
    n_c: Option<f64>,
    n_en: Option<f64>,
    l_f: Option<f64>,
    b_w: Option<f64>,
    cg: Option<Point<f64>>,
}

impl Component for Instruments {
    fn new(params: &Params) -> Self {
        Self {
            k_r: params.get("k_r").copied(),
            n_c: params.get("n_c").copied(),
            n_en: params.get("n_en").copied(),
            l_f: params.get("l_f").copied(),
            b_w: params.get("b_w").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_instruments").copied(),
                params.get("y_cg_instruments").copied(),
                params.get("z_cg_instruments").copied(),
            )),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            4.509 *
            self.k_r? *
            self.n_c?.powf(0.541) *
            self.n_en? *
            f64::powf(self.l_f? + self.b_w?, 0.5)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}

//------------------------------------------ hydrolic system

#[derive(Clone, Copy)]
struct HydrolicSystem {
    n_f: Option<f64>,
    l_f: Option<f64>,
    b_w: Option<f64>,
    cg: Option<Point<f64>>,
}

impl Component for HydrolicSystem {
    fn new(params: &Params) -> Self {
        Self {
            n_f: params.get("n_f").copied(),
            l_f: params.get("l_f").copied(),
            b_w: params.get("b_w").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_hydrolic_system").copied(),
                params.get("y_cg_hydrolic_system").copied(),
                params.get("z_cg_hydrolic_system").copied(),
            )),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            0.2673 *
            self.n_f? *
            f64::powf(self.l_f? + self.b_w?, 0.937)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}

//------------------------------------------ electrical systems

#[derive(Clone, Copy)]
struct ElectricalSystems {
    r_kva: Option<f64>,
    l_a: Option<f64>,
    n_gen: Option<f64>,
    cg: Option<Point<f64>>,
}

impl Component for ElectricalSystems {
    fn new(params: &Params) -> Self {
        Self {
            r_kva: params.get("r_kva").copied(),
            l_a: params.get("l_a").copied(),
            n_gen: params.get("n_gen").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_electrical_systems").copied(),
                params.get("y_cg_electrical_systems").copied(),
                params.get("z_cg_electrical_systems").copied(),
            )),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            7.291 *
            self.r_kva?.powf(0.782) *
            self.l_a?.powf(0.346) *
            self.n_gen?.powf(0.1)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}

//------------------------------------------ avionics

#[derive(Clone, Copy)]
struct Avionics {
    w_uav: Option<f64>,
    cg: Option<Point<f64>>,
}

impl Component for Avionics {
    fn new(params: &Params) -> Self {
        Self {
            w_uav: params.get("w_uav").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_avionics").copied(),
                params.get("y_cg_avionics").copied(),
                params.get("z_cg_avionics").copied(),
            )),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(1.73 * self.w_uav?.powf(0.983)))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}

//------------------------------------------ furnishing

#[derive(Clone, Copy)]
struct Furnishing {
    n_c: Option<f64>,
    w_c: Option<f64>,
    s_f: Option<f64>,
    n_seat: Option<f64>,
    w_seat: Option<f64>,
    k_lav: Option<f64>,
    n_p: Option<f64>,
    k_buf: Option<f64>,
    cg: Option<Point<f64>>,
}

impl Component for Furnishing {
    fn new(params: &Params) -> Self {
        Self {
            n_c: params.get("n_c").copied(),
            w_c: params.get("w_c").copied(),
            s_f: params.get("s_f").copied(),
            n_seat: params.get("n_seat").copied(),
            w_seat: params.get("w_seat").copied(),
            k_lav: params.get("k_lav").copied(),
            n_p: params.get("n_p").copied(),
            k_buf: params.get("k_buf").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_furnishing").copied(),
                params.get("y_cg_furnishing").copied(),
                params.get("z_cg_furnishing").copied(),
            )),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            0.0577 * self.n_c?.powf(0.1) *
            self.w_c?.powf(0.393) * self.s_f?.powf(0.75) +
            self.n_seat? * self.w_seat? +
            self.k_lav? * self.n_p?.powf(1.33) +
            self.k_buf? * self.n_p?.powf(1.12)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}

//------------------------------------------ air conditioning

#[derive(Clone, Copy)]
struct AirConditioning {
    n_p: Option<f64>,
    v_pr: Option<f64>,
    w_uav: Option<f64>,
    cg: Option<Point<f64>>,
}

impl Component for AirConditioning {
    fn new(params: &Params) -> Self {
        Self {
            n_p: params.get("n_p").copied(),
            v_pr: params.get("v_pr").copied(),
            w_uav: params.get("w_uav").copied(),
            cg: Point::move_option(Point::new(
                params.get("x_cg_air_conditioning").copied(),
                params.get("y_cg_air_conditioning").copied(),
                params.get("z_cg_air_conditioning").copied(),
            )),
        }
    }

    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            62.36 *
            self.n_p?.powf(0.25) *
            f64::powf(self.v_pr? * 0.001, 0.604) *
            self.w_uav?.powf(0.1)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}

//------------------------------------------ anti icing

#[derive(Clone, Copy)]
struct AntiIcing {
    cg: Option<Point<f64>>,
}

impl Component for AntiIcing {
    fn new(params: &Params) -> Self {
        Self {
            cg: Point::move_option(Point::new(
                params.get("x_cg_anti_icing").copied(),
                params.get("y_cg_anti_icing").copied(),
                params.get("z_cg_anti_icing").copied(),
            )),
        }
    }

    /// - w_dg: design gross weight in lb
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(0.002 * w_dg))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}

//------------------------------------------ handling gear

#[derive(Clone, Copy)]
struct HandlingGear {
    cg: Option<Point<f64>>,
}

impl Component for HandlingGear {
    fn new(params: &Params) -> Self {
        Self {
            cg: Point::move_option(Point::new(
                params.get("x_cg_handling_gear").copied(),
                params.get("y_cg_handling_gear").copied(),
                params.get("z_cg_handling_gear").copied(),
            )),
        }
    }

    /// - w_dg: design gross weight in lb
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(0.0003 * w_dg))
    }

    fn cg(self) -> Option<Point<f64>> {
        self.cg
    }
}
