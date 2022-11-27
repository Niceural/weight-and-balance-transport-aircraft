pub mod components;
pub mod utils;
pub mod stability;

use std::collections::HashMap;
use std::error::Error;
use csv::Reader;
use crate::components::wings::Wings;
use crate::components::tailplane::Tailplane;
use crate::components::fuselage::Fuselage;
use crate::utils::*;
use utils::point::Point;
use crate::components::varying::*;

const WEIGHTS_FILE_PATH: &str = "./data/weights.csv"; // relative path to parameters file
const BALANCE_FILE_PATH: &str = "./data/balance2.csv";
type Params = HashMap<String, f64>;

fn main() -> Result<(), Box<dyn Error>> {
    let params: Params = read_params_files()?;

    // empty weight
    let wings = Wings::new(&params);    
    let fuselage = Fuselage::new(&params);
    let tailplane = Tailplane::new(&params);

    // varying weights
    let pilots = Pilots::new(2., 85.);
    let crew = Crew::new(3., 85.);
    let passengers = Passengers::new(90., 100., LoadCase::Center);

    let w_dg = kg_to_lb(38249.);

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

fn read_params_files() -> Result<Params, Box<dyn Error>> {
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
    Ok(params)
}

