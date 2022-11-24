use crate::utils::point::Point;
use crate::Params;
use crate::components::Component;
use crate::utils::weight::Weight;

pub struct Tailplane {
    ht: HorizontalTailplane,
    vt: VerticalTailplane,
}

impl Component for Tailplane {
    fn new(params: &Params) -> Self {
        let ht = HorizontalTailplane::new(params);
        let vt = VerticalTailplane::new(params);
        Self { ht, vt }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(
            self.ht.weight(w_dg, n_z)? +
            self.vt.weight(w_dg, n_z)?
        )
    }

    fn cg(self) -> Option<Point<f64>> {
        Some(Point::new(0., 0., 0.))
    }
}

pub struct HorizontalTailplane {
    // for weight
    k_uht: Option<f64>,    // 1.143 for all-moving tail, 1.0 otherwise
    s_ht: Option<f64>,     // horizontal tailplane area in ft^2
    k_y: Option<f64>,      // aircraft pitching radius of giration, approx 0.3L_ht in ft
    ar_h: Option<f64>,     // Horizontal tailplane aspect ratio
    s_e: Option<f64>,      // Elevator area (ft2)
    f_w: Option<f64>,      // Fuselage width at horizontal tail intersection (ft)
    b_ht: Option<f64>,      // Horizontal tailplane b_h (ft)
    l_ht: Option<f64>, // l_ht from wing aerodynamic centre to horizontal tailplane aerodynamic centre (ft)
    sweep_ht: Option<f64>, // Horizontal tailplane quarter chord sweep_ht in radians
    // for balance
    root_ht: Option<Point<f64>>, // position of the root of the horizontal tailplane
    chord_ht: Option<f64>, // leading edge to trailing edge distance in ft
    aoa_ht: Option<f64>, // horizontal tailplane angle of attack
}

impl Component for HorizontalTailplane {
    fn new(params: &Params) -> Self {
        Self {
            k_uht: params.get("k_uht").copied(),
            s_ht: params.get("s_ht").copied(),
            k_y: params.get("k_y").copied(),
            ar_h: params.get("ar_h").copied(),
            s_e: params.get("s_e").copied(),
            f_w: params.get("f_w").copied(),
            b_ht: params.get("b_ht").copied(),
            l_ht: params.get("l_ht").copied(),
            sweep_ht: params.get("sweep_ht").copied(),
            root_ht: Point::move_option(Point::new(
                params.get("x_root_ht").copied(),
                params.get("y_root_ht").copied(),
                params.get("z_root_ht").copied(),
            )),
            chord_ht: params.get("chord_ht").copied(),
            aoa_ht: params.get("aoa_ht").copied(),
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            // numerator
            0.0379 *
            self.k_uht? *
            f64::powf(w_dg, 0.639) *
            f64::powf(n_z, 0.1) *
            f64::powf(self.s_ht?, 0.75) *
            f64::powf(self.k_y?, 0.704) *
            f64::powf(self.ar_h?, 0.166) *
            f64::powf(1.0 + self.s_e? / self.s_ht?, 0.1) /
            // denominator
            f64::powf(1. + self.f_w? / self.b_ht?, 0.25) /
            self.l_ht? /
            f64::cos(self.sweep_ht?)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        let x = self.root_ht?.x() * 0.42;
        let cg = Point::new(x, 0., self.root_ht?.z() - x * self.aoa_ht?.tan());
        Some(self.root_ht? + cg)
    }
}

struct VerticalTailplane {
    // weight
    h_t: Option<f64>,// Location of horizontal tailplane on vertical tail. 0.0 for fuselage mounted horizontal tail; 1.0 for T-tail
    h_v: Option<f64>,
    s_vt: Option<f64>, // Vertical tailplane area (ft2)
    k_z: Option<f64>, // Aircraft yaw radius of gyration approx L_vt (ft)
    ar_v: Option<f64>, // Vertical tailplane aspect ratio
    l_vt: Option<f64>, // Length from wing aerodynamic centre to vertical tailplane aerodynamic centre (ft)
    sweep_vt: Option<f64>, // Vertical tailplane quarter chord sweep
    t_c_ratio_root_v: Option<f64>, // Vertical tailplane root thickness to chord ratio
    // balance
    root_vt: Option<Point<f64>>, // point where the vertival tailplane is attached to the fuselage
    chord_55_vt: Option<f64>, // chord length at 55% of the fin height from the root chord
    fin_height_vt: Option<f64>, // fin height in ft
}

impl Component for VerticalTailplane {
    fn new(params: &Params) -> Self {
        Self {
            // weight
            h_t: params.get("h_t").copied(),
            h_v: params.get("h_v").copied(),
            s_vt: params.get("s_vt").copied(),
            k_z: params.get("k_z").copied(),
            ar_v: params.get("ar_v").copied(),
            l_vt: params.get("l_vt").copied(),
            sweep_vt: params.get("sweep_vt").copied(),
            t_c_ratio_root_v: params.get("t_c_ratio_root_v").copied(),
            // balance
            root_vt: Point::move_option(Point::new(
                params.get("x_root_vt").copied(),
                params.get("y_root_vt").copied(),
                params.get("z_root_vt").copied(),
            )),
            chord_55_vt: params.get("chord_55_vt").copied(),
            fin_height_vt: params.get("fin_height_vt").copied(),
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            // numerator
            0.0026 *
            f64::powf(1. + self.h_t? / self.h_v?, 0.225) *
            w_dg.powf(0.556) *
            n_z.powf(0.536) *
            self.s_vt?.powf(0.5) *
            self.k_z?.powf(0.875) *
            self.ar_v?.powf(0.35) /
            // denominator
            self.l_vt?.powf(0.5) /
            self.sweep_vt?.cos() /
            self.t_c_ratio_root_v?.powf(0.5)
        ))
    }

    fn cg(self) -> Option<Point<f64>> {
        let z = 0.55 * self.fin_height_vt?;
        let cg = Point::new(0.42 * self.chord_55_vt? + z * self.sweep_vt?.tan(), 0., z);
        Some(self.root_vt? + cg)
    }
}

