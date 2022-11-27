pub mod components;
pub mod utils;
pub mod stability;

use std::collections::HashMap;
use std::error::Error;
use csv::Reader;
// use crate::components::{fuselage::Fuselage, wings::Wings, tailplane::Tailplane};
use crate::components::wings::Wings;
use crate::components::tailplane::Tailplane;
use crate::components::fuselage::Fuselage;
use utils::point::Point;

const WEIGHTS_FILE_PATH: &str = "./data/parameters1.csv"; // relative path to parameters file
const BALANCE_FILE_PATH: &str = "./data/balance1.csv";
type Params = HashMap<String, f64>;

fn main() -> Result<(), Box<dyn Error>> {
    let mut params: Params = HashMap::new();
    let mut reader = Reader::from_path(WEIGHTS_FILE_PATH)?;
    for result in reader.records() {
        let record = result?;
        let symbol = record[0].to_string();
        let value = record[1].parse::<f64>()?;
        params.insert(symbol, value);
    }
    let mut reader_balance = Reader::from_path(BALANCE_FILE_PATH)?;
    for result in reader_balance.records() {
        let record = result?;
        let symbol = record[0].to_string();
        let value = record[1].parse::<f64>()?;
        params.insert(symbol, value);
    }

    let wings = Wings::new(&params);    
    let fuselage = Fuselage::new(&params);
    let tailplane = Tailplane::new(&params);

    let w_dg = 84350.;

    let total_weight = wings.weight(w_dg) +
        fuselage.weight(w_dg) +
        tailplane.weight(w_dg);

    let cg = (
            wings.pos_times_weight(w_dg, Point::new(28.2, 0., 0.)) +
            tailplane.pos_times_weight(w_dg) +
            fuselage.pos_times_weight(w_dg)
        ) / total_weight.get_val();
    

    println!("total weight: {} lb", total_weight.get_val());
    println!("cg: {}", cg.x());

    Ok(())
}

