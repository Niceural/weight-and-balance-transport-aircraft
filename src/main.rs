pub mod components;
pub mod utils;
use components::Aircraft;
use std::collections::HashMap;
use std::process;
use crate::utils::extract_params;

const PARAMS_FILE_PATH: &str = "./parameters.csv"; // relative path to parameters file
type Params = HashMap<String, f64>;

fn main() {
    let params: Params = match extract_params(PARAMS_FILE_PATH) {
        Ok(p) => p,
        Err(err) => { println!("{}", err); process::exit(1); },
    };

    let aircraft = Aircraft::new(&params);
}

