use crate::utils::coordinate::Coordinate;

pub struct Fuselage {
    // weight
    k_door: f64, // 1.0 if no cargo door; 1.06 for one side cargo door; 1.12 for two side cargo doors; 1.12 for aft clamshell door; 1.25 for two side and an aft clamshell cargo doors
    k_lg: f64, // 1.12 for fuselage mounted landing gear; 1.0 otherwise
    l: f64, // Fuselage structural length (ft)
    s_f: f64, // Fuselage wetted area (ft2)
    k_ws: f64, // 0.75[(1 + 2λ)/(1 + λ)]Bw tan Λ/L
    d: f64, // Maximum fuselage diameter (ft)
    // balance
    pos_cg: f64, // horizontal CG position of the fuselage (given as % fuselage length and measured from the nose), 42 −45% for wing mounted engines
}

impl Fuselage {
    pub fn new() -> Self {
        Self {
            k_door: 9.90,
            k_lg: 9.90,
            l: 9.90,
            s_f: 9.90,
            k_ws: 9.90,
            d: 9.90,
            pos_cg: 9.90,
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        let mut r = 0.3280;
        r *= self.k_door;
        r *= self.k_lg;
        r *= f64::powf(w_dg * n_z, 0.5);
        r *= self.l.powf(0.25);
        r *= self.s_f.powf(0.302);
        r *= f64::powf(1. + self.k_ws, 0.04);
        r *= f64::powf(self.l / self.d, 0.1);
        if r < 0. { eprintln!("negative weight"); }
        r
    }

    pub fn cg(self) -> Coordinate {
        Coordinate::new( self.pos_cg * self.l, 0., 0.)
    }
}