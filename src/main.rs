pub mod components;
pub mod utils;
use components::{
    engines::Engines, fuselage::Fuselage, horizontal_tailplane::HorizontalTailplane,
    landing_gear::LandingGear, systems::Systems, vertical_tailplane::VerticalTailplace,
    wings::Wings,
};
#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;
use std::fs::File;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("my_rust_binary.log").unwrap(),
        ),
    ])
    .unwrap();

    let w_dg = 9.90; // Design gross weight (lb)
    let n_z = 9.90; // Ultimate load factor; 1.5× limit load factor

    let wings = Wings::new();
    let horizontal_tailplane = HorizontalTailplane::new();
    let vertical_tailplace = VerticalTailplace::new();
    let fuselage = Fuselage::new();
    let landing_gear = LandingGear::new();
    let engines = Engines::new();
    let systems = Systems::new();

    let w = wings.weight(w_dg, n_z)
        + horizontal_tailplane.weight(w_dg, n_z)
        + vertical_tailplace.weight(w_dg, n_z)
        + fuselage.weight(w_dg, n_z)
        + landing_gear.weight()
        + engines.weight(n_z)
        + systems.weight(w_dg);

    println!("Final weight: {} lb", w);
}
