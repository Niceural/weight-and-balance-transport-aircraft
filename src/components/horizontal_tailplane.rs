pub struct HorizontalTailplane {
    k_uht: f64, // 1.143 for all-moving tail, 1.0 otherwise
    area: f64, // horizontal tailplane area in ft^2
    k_y: f64, // aircraft pitching radius of giration, approx 0.3L_ht in ft
    ar: f64, // Horizontal tailplane aspect ratio
    elevator_area: f64, // Elevator area (ft2)
    fuselage_width: f64, // Fuselage width at horizontal tail intersection (ft)
    span: f64, // Horizontal tailplane span (ft)
    length: f64, // Length from wing aerodynamic centre to horizontal tailplane aerodynamic centre (ft)
    sweep: f64, // Horizontal tailplane quarter chord sweep in radians
}

impl HorizontalTailplane {
    pub fn new() -> Self {
        Self {
            k_uht: 1.0,
            area: 9.99,
            k_y: 9.99,
            ar: 9.99,
            elevator_area: 9.99,
            fuselage_width: 9.99,
            span: 9.99,
            length: 9.99,
            sweep: 9.99,
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
        r *= f64::powf(self.area, 0.75);
        r *= f64::powf(self.k_y, 0.704);
        r *= f64::powf(self.ar, 0.166);
        r *= f64::powf(1.0 + self.elevator_area / self.area, 0.1);
        // denominator
        r /= f64::powf(1. + self.fuselage_width / self.span, 0.25);
        r /= self.length;
        r /= f64::cos(self.sweep);
        r
    }
}