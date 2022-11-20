use crate::utils::coordinate::Coordinate;
use crate::Params;

pub struct Tailplane {
    ht: HorizontalTailplane,
    vt: VerticalTailplane,
}

impl Tailplane {
    pub fn new(params: &Params) -> Self {
        let ht = HorizontalTailplane::new(params);
        let vt = VerticalTailplane::new(params);
        Self { ht, vt, }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        let r = self.ht.weight(w_dg, n_z)
            + self.vt.weight(w_dg, n_z);
        if r < 0. { panic!("negative weight"); }
        r
    }
}

struct HorizontalTailplane {
    // for weight
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
    root_chord_ht: f64, // leading edge to trailing edge distance in ft
}

impl HorizontalTailplane {
    pub fn new(params: &Params) -> Self {
        Self {
            k_uht: params.get("s_w").unwrap().clone(),
            s_ht: params.get("s_ht").unwrap().clone(),
            k_y: params.get("k_y").unwrap().clone(),
            ar_h: params.get("ar_h").unwrap().clone(),
            s_e: params.get("s_e").unwrap().clone(),
            f_w: params.get("f_w").unwrap().clone(),
            b_ht: params.get("b_ht").unwrap().clone(),
            l_ht: params.get("l_ht").unwrap().clone(),
            sweep_ht: params.get("sweep_ht").unwrap().clone(),
            root_chord_ht: params.get("root_chord_ht").unwrap().clone(),
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        // numerator
        let mut r = 0.0379;
        r *= self.k_uht;
        r *= f64::powf(w_dg, 0.639);
        r *= f64::powf(n_z, 0.1);
        r *= f64::powf(self.s_ht, 0.75);
        r *= f64::powf(self.k_y, 0.704);
        r *= f64::powf(self.ar_h, 0.166);
        r *= f64::powf(1.0 + self.s_e / self.s_ht, 0.1);
        // denominator
        r /= f64::powf(1. + self.f_w / self.b_ht, 0.25);
        r /= self.l_ht;
        r /= f64::cos(self.sweep_ht);
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self, coord_le_rc: Coordinate) -> Coordinate {
        let mut r = coord_le_rc;
        r.x += 0.42 * self.root_chord_ht;
        r
    }
}

struct VerticalTailplane {
    // weight
    h_t: f64,// Location of horizontal tailplane on vertical tail. 0.0 for fuselage mounted horizontal tail; 1.0 for T-tail
    h_v: f64,
    s_vt: f64, // Vertical tailplane area (ft2)
    k_z: f64, // Aircraft yaw radius of gyration approx L_vt (ft)
    ar_v: f64, // Vertical tailplane aspect ratio
    l_vt: f64, // Length from wing aerodynamic centre to vertical tailplane aerodynamic centre (ft)
    sweep_vt: f64, // Vertical tailplane quarter chord sweep
    t_c_ratio_root_v: f64, // Vertical tailplane root thickness to chord ratio
    // balance
    root_chord_vt: f64, // root chord in ft
    fin_height_vt: f64, // fin height in ft
}

impl VerticalTailplane {
    pub fn new(params: &Params) -> Self {
        Self {
            // weight
            h_t: params.get("h_t").unwrap().clone(),
            h_v: params.get("h_v").unwrap().clone(),
            s_vt: params.get("s_vt").unwrap().clone(),
            k_z: params.get("k_z").unwrap().clone(),
            ar_v: params.get("ar_v").unwrap().clone(),
            l_vt: params.get("l_vt").unwrap().clone(),
            sweep_vt: params.get("sweep_vt").unwrap().clone(),
            t_c_ratio_root_v: params.get("t_c_ratio_root_v").unwrap().clone(),
            // balance
            root_chord_vt: params.get("root_chord_vt").unwrap().clone(),
            fin_height_vt: params.get("fin_height_vt").unwrap().clone(),
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        // numerator
        let mut r = 0.0026;
        r *= f64::powf(1. + self.h_t / self.h_v, 0.225);
        r *= w_dg.powf(0.556);
        r *= n_z.powf(0.536);
        r *= self.s_vt.powf(0.5);
        r *= self.k_z.powf(0.875);
        r *= self.ar_v.powf(0.35);
        // denominator
        r /= self.l_vt.powf(0.5);
        r /= self.sweep_vt.cos();
        r /= self.t_c_ratio_root_v.powf(0.5);
        if r < 0. { panic!("negative weight"); }
        r
    }

    pub fn cg(self, root_le_pos: Coordinate) -> Coordinate {
        let mut r = root_le_pos;
        r.x = 0.42 * self.root_chord_vt;
        r.z = 0.55 * self.fin_height_vt;
        r
    }
}

