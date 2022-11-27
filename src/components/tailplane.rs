use crate::utils::point::Point;
use crate::Params;
use crate::utils::weight::Weight;

pub struct Tailplane {
    ht: HorizontalTailplane,
    vt: VerticalTailplane,
}

impl Tailplane {
    pub fn new(params: &Params) -> Self {
        Self {
            ht: HorizontalTailplane::new(params),
            vt: VerticalTailplane::new(params),
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(&self, w_dg: f64) -> Weight {
        self.ht.weight(w_dg) +
        self.vt.weight(w_dg)
    }

    pub fn pos_times_weight(&self, w_dg: f64) -> Point<f64> {
        self.ht.cg() * self.ht.weight(w_dg).get_val() +
        self.vt.cg() * self.vt.weight(w_dg).get_val()
    }
}

struct HorizontalTailplane {
    // for weight
    n_z: f64,
    k_uht: f64,    // 1.143 for all-moving tail, 1.0 otherwise
    s_ht: f64,     // horizontal tailplane area in ft^2
    k_y: f64,      // aircraft pitching radius of giration, approx 0.3L_ht in ft
    ar_h: f64,     // Horizontal tailplane aspect ratio
    s_e: f64,      // Elevator area (ft2)
    f_w: f64,      // Fuselage width at horizontal tail intersection (ft)
    b_ht: f64,      // Horizontal tailplane b_h (ft)
    l_ht: f64, // l_ht from wing aerodynamic centre to horizontal tailplane aerodynamic centre (ft)
    sweep_ht: f64, // Horizontal tailplane quarter chord sweep_ht in radians
    // for balance
    root_ht: Point<f64>, // position of the root of the horizontal tailplane
    chord_ht: f64, // leading edge to trailing edge distance in ft
    aoa_ht: f64, // horizontal tailplane angle of attack
}

impl HorizontalTailplane {
    pub fn new(params: &Params) -> Self {
        Self {
            n_z: params.get("n_z").expect("missing n_z").clone(),
            k_uht: params.get("k_uht").expect("missing k_uht").clone(),
            s_ht: params.get("s_ht").expect("missing s_ht").clone(),
            k_y: params.get("k_y").expect("missing k_y").clone(),
            ar_h: params.get("ar_h").expect("missing ar_h").clone(),
            s_e: params.get("s_e").expect("missing s_e").clone(),
            f_w: params.get("f_w").expect("missing f_w").clone(),
            b_ht: params.get("b_ht").expect("missing b_ht").clone(),
            l_ht: params.get("l_ht").expect("missing l_ht").clone(),
            sweep_ht: params.get("sweep_ht").expect("missing sweep_ht").clone(),
            root_ht: Point::new(
                params.get("x_root_ht").expect("missing x_root_ht").clone(),
                params.get("y_root_ht").expect("missing y_root_ht").clone(),
                params.get("z_root_ht").expect("missing z_root_ht").clone(),
            ),
            chord_ht: params.get("chord_ht").expect("missing chord_ht").clone(),
            aoa_ht: params.get("aoa_ht").expect("missing aoa_ht").clone(),
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(&self, w_dg: f64) -> Weight {
        Weight::new(
            // numerator
            0.0379 *
            self.k_uht *
            f64::powf(w_dg, 0.639) *
            f64::powf(self.n_z, 0.1) *
            f64::powf(self.s_ht, 0.75) *
            f64::powf(self.k_y, 0.704) *
            f64::powf(self.ar_h, 0.166) *
            f64::powf(1.0 + (self.s_e) / self.s_ht, 0.1) /
            // denominator
            f64::powf(1. + self.f_w / self.b_ht, 0.25) /
            self.l_ht /
            f64::cos(self.sweep_ht)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        let x = self.root_ht.x() * 0.42;
        let cg = Point::new(x, 0., self.root_ht.z() - x * self.aoa_ht.tan());
        Point::new(self.root_ht.x() + cg.x(), self.root_ht.y() + cg.y(), self.root_ht.z() + cg.z())
    }
}

struct VerticalTailplane {
    // weight
    n_z: f64,
    h_t: f64,// Location of horizontal tailplane on vertical tail. 0.0 for fuselage mounted horizontal tail; 1.0 for T-tail
    h_v: f64,
    s_vt: f64, // Vertical tailplane area (ft2)
    k_z: f64, // Aircraft yaw radius of gyration approx L_vt (ft)
    ar_v: f64, // Vertical tailplane aspect ratio
    l_vt: f64, // Length from wing aerodynamic centre to vertical tailplane aerodynamic centre (ft)
    sweep_vt: f64, // Vertical tailplane quarter chord sweep
    t_c_ratio_root_v: f64, // Vertical tailplane root thickness to chord ratio
    // balance
    root_vt: Point<f64>, // point where the vertival tailplane is attached to the fuselage
    chord_55_vt: f64, // chord length at 55% of the fin height from the root chord
    fin_height_vt: f64, // fin height in ft
}

impl VerticalTailplane{
    pub fn new(params: &Params) -> Self {
        Self {
            // weight
            n_z: params.get("n_z").expect("missing n_z").clone(),
            h_t: params.get("h_t").expect("missing h_t").clone(),
            h_v: params.get("h_v").expect("missing h_v").clone(),
            s_vt: params.get("s_vt").expect("missing s_vt").clone(),
            k_z: params.get("k_z").expect("missing k_z").clone(),
            ar_v: params.get("ar_v").expect("missing ar_v").clone(),
            l_vt: params.get("l_vt").expect("missing l_vt").clone(),
            sweep_vt: params.get("sweep_vt").expect("missing sweep_vt").clone(),
            t_c_ratio_root_v: params.get("t_c_ratio_root_v").expect("missing t_c_ratio_root_v").clone(),
            // balance
            root_vt: Point::new(
                params.get("x_root_vt").expect("missing x_root_vt").clone(),
                params.get("y_root_vt").expect("missing y_root_vt").clone(),
                params.get("z_root_vt").expect("missing z_root_vt").clone(),
            ),
            chord_55_vt: params.get("chord_55_vt").expect("missing chord_55_vt").clone(),
            fin_height_vt: params.get("fin_height_vt").expect("missing fin_height_vt").clone(),
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(&self, w_dg: f64) -> Weight {
        Weight::new(
            // numerator
            0.0026 *
            f64::powf(1. + self.h_t / self.h_v, 0.225) *
            w_dg.powf(0.556) *
            self.n_z.powf(0.536) *
            self.s_vt.powf(0.5) *
            self.k_z.powf(0.875) *
            self.ar_v.powf(0.35) /
            // denominator
            self.l_vt.powf(0.5) /
            self.sweep_vt.cos() /
            self.t_c_ratio_root_v.powf(0.5)
        )
    }

    pub fn cg(&self) -> Point<f64> {
        let z = 0.55 * self.fin_height_vt;
        let cg = Point::new(0.42 * self.chord_55_vt + z * self.sweep_vt.tan(), 0., z);
        Point::new(self.root_vt.x() + cg.x(), self.root_vt.y() + cg.y(), self.root_vt.z() + cg.z())
    }
}

