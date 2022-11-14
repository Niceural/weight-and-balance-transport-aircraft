pub struct Nacelle {
    k_ng: f64, // 1.017 for pylon mounted nacelle, 1.0 otherwise
    length: f64, // Nacelle length (ft)
    width: f64, // Nacelle width (ft)
    w_enc: f64, // Weight of engine and contents in lb (see pdf)
    num_of_engines: f64, // Number of engines
    wetted_area: f64, // Nacelle wetted area (ft^2)
}

impl Nacelle {
    pub fn new() -> Self {
        Self {
            k_ng: 1.0,
            length: 9.99,
            width: 9.99,
            w_enc: 9.99,
            num_of_engines: 2.0,
            wetted_area: 9.99,
        }
    }

    /// - w_dg: design gross weight in lb
    /// - n_z: ultimate load factor, 1.5x limit load factor
    pub fn weight(self, w_dg: f64, n_z: f64) -> f64 {
        let mut r: f64 = 0.6724;
        r *= self.k_ng;
        r *= f64::powf(self.length, 0.1);
        r *= f64::powf(self.width, 0.294);
        r *= f64::powf(n_z, 0.119);
        r *= f64::powf(self.w_enc, 0.611);
        r *= f64::powf(self.num_of_engines, 0.984);
        r *= f64::powf(self.wetted_area, 0.224);
        r
    }
}