pub mod coordinate;
use std::collections::HashMap;
use std::error::Error;
use csv::Reader;
use crate::Params;

pub fn extract_params(path: &str) -> Result<Params, Box<dyn Error>> {
    let mut params: Params = HashMap::new();
    let mut reader = match Reader::from_path(path) {
        Err(err) => return Err(From::from(err)),
        Ok(rdr) => rdr,
    };
    for (index, result) in reader.records().enumerate() {
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                let symbol = record[0].to_string();
                let value = match record[1].parse::<f64>() {
                    Err(err) => return Err(From::from(err)),
                    Ok(val) => val,
                };
                params.insert(symbol, value);
            }
        }
    }
    Ok(params)
}

pub fn kg_to_lb(kg: f64) -> f64 {
    kg * 2.20462
}

pub fn m2_to_ft2(m2: f64) -> f64 {
    m2 * 10.7639
}

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * 0.0174533
}
