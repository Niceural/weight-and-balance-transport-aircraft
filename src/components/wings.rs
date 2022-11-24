use crate::utils::point::Point;
use crate::utils::weight::Weight;
use crate::components::Component;
use crate::Params;

pub struct Wings {
    // weights
    s_w: Option<f64>,            // reference wing area in ft^2
    ar: Option<f64>,             // wing aspect ratio
    lambda: Option<f64>,         // wing taper ratio
    s_csw: Option<f64>,          // area of wing mounted control surfaces in ft^2
    t_c_ratio_root: Option<f64>, // wing root thickness to chord ratio
    sweep: Option<f64>,        // wing quarter chord sweep in rad
    // balance
    delta_fs_as: Option<f64>, // distance between forward spar and aft spar at centerline in ft
    x_fs: Option<f64>,        // centerline forward spar position from quarter chord in ft
    sweep_fs: Option<f64>,    // forward spar sweep in radians
    sweep_as: Option<f64>,    // aft spar sweep in radians
    dihedral: Option<f64>,    // wing quarter chord dihedral in radians
    wing_span: Option<f64>,   // wing span in ft
}

impl Component for Wings {
    fn new(params: &Params) -> Self {
        Self {
            // weights
            s_w: params.get("s_w").copied(),
            ar: params.get("ar").copied(),
            lambda: params.get("lambda").copied(),
            s_csw: params.get("s_csw").copied(),
            t_c_ratio_root: params.get("t_c_ratio_root").copied(),
            sweep: params.get("sweep").copied(),
            // balance
            delta_fs_as: params.get("delta_fs_as").copied(),
            x_fs: params.get("x_fs").copied(),
            sweep_fs: params.get("sweep_fs").copied(),
            sweep_as: params.get("sweep_as").copied(),
            dihedral: params.get("dihedral").copied(),
            wing_span: params.get("wing_span").copied(),
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    fn weight(self, w_dg: f64, n_z: f64) -> Option<Weight> {
        Some(Weight::new(
            // numerator
            0.0051 *
            f64::powf(w_dg * n_z, 0.557) *
            self.s_w?.powf(0.649) *
            self.ar?.powf(0.5) *
            f64::powf(1.0 + self.lambda?, 0.1) *
            self.s_csw?.powf(0.1) /
            // denominator
            self.sweep?.cos() /
            self.t_c_ratio_root?.powf(0.4)
        ))
    }

    /// - pos_w: position of quarter chord point in ft
    fn cg(self) -> Option<Point<f64>> {
        // 35% semi span from centerline
        let y = 0.35 * self.wing_span? * 0.5;
        let z = y * self.dihedral?.tan();
        // opposite sides of triangles
        let opposite_fs = y * self.sweep_fs?.tan();
        let opposite_as = y * self.sweep_as?.tan();
        let difference_07 = (self.delta_fs_as? + opposite_as - opposite_fs) * 0.7;
        let x = difference_07 + opposite_fs + self.x_fs?;
        Some(Point::new(x, 0., z))
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::components::wings::Wings;
    use crate::utils::coordinate::Point;

    #[test]
    fn negative_weight() {
        let wings = Wings::new();
        assert!(0. < wings.weight());
    }

    #[test]
    fn x_cg() {
        let chord_pos = Point::new(17.4, 0., 0.9);
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
        let chord_pos = Point::new(13.7, 0.0, 1.3);
        let wings = Wings::new();
        let cg_actual = wings.cg(chord_pos);
        assert_eq!(cg_actual.y, 0.);
    }

    #[test]
    fn z_cg() {
        let chord_pos = Point::new(19.57, 0.0, 4.28);
        let wings = Wings::new();
        let span_35 = wings.wing_span * 0.5 * 0.35;
        let z_expected = wings.dihedral.tan() * span_35 + chord_pos.z;
        let cg_actual = wings.cg(chord_pos);
        assert_eq!(cg_actual.z, z_expected);
    }
}
*/
