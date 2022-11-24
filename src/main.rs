pub mod components;
pub mod utils;

use std::collections::HashMap;
use std::error::Error;
use csv::Reader;
use crate::components::{Component, wings::Wings, tailplane::Tailplane, systems::Systems, landing_gear::LandingGear, fuselage::Fuselage, engines::Engines};

const PARAMS_FILE_PATH: &str = "./data/parameters1.csv"; // relative path to parameters file
type Params = HashMap<String, f64>;

fn main() -> Result<(), Box<dyn Error>> {
    let mut params: Params = HashMap::new();
    let mut reader = Reader::from_path(PARAMS_FILE_PATH)?;
    for (index, result) in reader.records().enumerate() {
        let record = result?;
        let symbol = record[0].to_string();
        let value = record[1].parse::<f64>()?;
        params.insert(symbol, value);
    }

    let wings = Wings::new(&params);    
    let tailplane = Tailplane::new(&params);
    let systems = Systems::new(&params);
    let landing_gear = LandingGear::new(&params);
    let fuselage = Fuselage::new(&params);
    let engines = Engines::new(&params);

    let w_dg = 84350.;
    let n_z = 3.75;

    let total_weight = wings.weight(w_dg, n_z).expect("invalid parameters file") +
        tailplane.weight(w_dg, n_z).expect("invalid parameters file") +
        systems.weight(w_dg, n_z).expect("invalid parameters file") +
        landing_gear.weight(w_dg, n_z).expect("invalid parameters file") +
        fuselage.weight(w_dg, n_z).expect("invalid parameters file") +
        engines.weight(w_dg, n_z).expect("invalid parameters file");

    println!("total weight: {} lb", total_weight.get_val());

    Ok(())
}

