use crate::utils::coordinate::Coordinate;
use log::warn;

pub struct Wings {
    // weights
    pub s_w: f64,            // reference wing area in ft^2
    pub ar: f64,             // wing aspect ratio
    pub lambda: f64,         // wing taper ratio
    pub s_csw: f64,          // area of wing mounted control surfaces in ft^2
    pub t_c_ratio_root: f64, // wing root thickness to chord ratio
    pub sweep_w: f64,        // wing quarter chord sweep in rad
    // balance
    pub delta_fs_as: f64, // distance between forward spar and aft spar at centerline in ft
    pub x_fs: f64,        // centerline forward spar position from quarter chord in ft
    pub sweep_fs: f64,    // forward spar sweep in radians
    pub sweep_as: f64,    // aft spar sweep in radians
    pub dihedral: f64,    // wing quarter chord dihedral in radians
    pub wing_span: f64,   // wing span in ft
}

impl Wings {
    pub fn new() -> Self {
        Self {
            // weights
            s_w: 9.90,
            ar: 9.90,
            lambda: 9.90,
            s_csw: 9.90,
            t_c_ratio_root: 9.90,
            sweep_w: 9.90,
            // balance
            delta_fs_as: 9.90,
            x_fs: 9.90,
            sweep_fs: 9.90,
            sweep_as: 9.90,
            dihedral: 9.90,
            wing_span: 9.90,
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        // numerator
        let mut r: f64 = 0.0051;
        r *= f64::powf(w_dg * n_z, 0.557);
        r *= self.s_w.powf(0.649);
        r *= self.ar.powf(0.5);
        r *= f64::powf(1.0 + self.lambda, 0.1);
        r *= self.s_csw.powf(0.1);
        // denominator
        r /= self.sweep_w.cos();
        r /= self.t_c_ratio_root.powf(0.4);
        if r < 0. {
            warn!("negative weight");
        }
        r
    }

    /// - pos_w: position of quarter chord point in ft
    pub fn cg(self, pos_w: Coordinate) -> Coordinate {
        let mut r = Coordinate::new_origin();
        // 35% semi span from centerline
        r.y = 0.35 * self.wing_span * 0.5;
        r.z = r.y * self.dihedral.tan();
        // opposite sides of triangles
        let opposite_fs = r.y * self.sweep_fs.tan();
        let opposite_as = r.y * self.sweep_as.tan();
        let difference_07 = (self.delta_fs_as + opposite_as - opposite_fs) * 0.7;
        r.x = difference_07 + opposite_fs + self.x_fs;
        r.y = 0.;
        pos_w + r
    }
}

#[cfg(test)]
mod tests {
    use crate::components::wings::Wings;
    use crate::utils::coordinate::Coordinate;

    #[test]
    fn negative_weight() {
        let wings = Wings::new();
        assert!(0. < wings.weight());
    }

    #[test]
    fn x_cg() {
        let chord_pos = Coordinate::new(17.4, 0., 0.9);
        let wings = Wings::new();
        // 35% of the semi-span from the centreline
        let span_35 = wings.wing_span * 0.5 * 0.35;
        // 70% of the local distance between the forward and aft spars
        let opposite_fs = span_35 * wings.sweep_fs.tan();
        let opposite_as = span_35 * wings.sweep_as.tan();
        let difference = wings.delta_fs_as + opposite_as - opposite_fs;
        let expected_pos = difference * 0.7 + opposite_fs + wings.x_fs + chord_pos.x;
        let actual_cg = wings.cg(chord_pos);
        assert_eq!(actual_cg.x, expected_pos);
    }

    #[test]
    fn y_cg() {
        let chord_pos = Coordinate::new(13.7, 0.0, 1.3);
        let wings = Wings::new();
        let cg_actual = wings.cg(chord_pos);
        assert_eq!(cg_actual.y, 0.);
    }

    #[test]
    fn z_cg() {
        let chord_pos = Coordinate::new(19.57, 0.0, 4.28);
        let wings = Wings::new();
        let span_35 = wings.wing_span * 0.5 * 0.35;
        let z_expected = wings.dihedral.tan() * span_35 + chord_pos.z;
        let cg_actual = wings.cg(chord_pos);
        assert_eq!(cg_actual.z, z_expected);
    }
}
