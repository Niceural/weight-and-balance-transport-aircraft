pub mod components;
pub mod utils;
use csv;
use std::collections::HashMap;

const PARAMS_FILE_PATH: &str = "./parameters.csv";
type Params = HashMap<String, f64>;

fn main() {
    let params: Params = extract_params(PARAMS_FILE_PATH);
}

fn extract_params(path: &str) -> Params {
    let mut params_file = csv::Reader::from_path(path).unwrap();
    let mut params: Params = HashMap::new();
    for result in params_file.records() {
        let record = result.unwrap();
        params.insert(record[0].to_string(), record[1].parse::<f64>().unwrap());
    }
    params
}

