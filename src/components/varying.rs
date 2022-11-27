use crate::utils::weight::Weight;
use crate::utils::point::Point;
use crate::utils::in_to_ft;

pub struct Pilots {
    num_pilots: f64, // number of pilots
    weight_per_pilot: f64, // weight per pilot (lb)
    cg: Point<f64>,// cg of pilots
}

impl Pilots {
    pub fn new(num_pilots: f64, weight_per_pilot: f64) -> Self {
        Self { 
            num_pilots,
            weight_per_pilot,
            cg: Point::new(in_to_ft(39.37) + 2., 0., 0.), // seat position in cabin
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(self.num_pilots * self.weight_per_pilot)
    }

    pub fn pos_times_weight(&self) -> Point<f64> {
        self.cg * self.weight().get_val()
    }
}

pub struct Crew {
    num_crew: f64, // number of pilots
    weight_per_crew: f64, // weight per pilot (lb)
    cg: Point<f64>, // middle of fuselage
}

impl Crew {
    pub fn new(num_crew: f64, weight_per_crew: f64) -> Self {
        Self {
            num_crew,
            weight_per_crew,
            cg: Point::new(47., 0., 0.),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(self.num_crew * self.weight_per_crew)
    }

    pub fn pos_times_weight(&self) -> Point<f64> {
        self.cg * self.weight().get_val()
    }
}

pub enum LoadCase {
    Front,
    Rear,
    Center,
}

pub struct Passengers {
    num_passengers: f64, // number of passengers
    weight_per_passenger: f64, // weight per passenger (lb)
    load_case: LoadCase, // loaded from front or rear or center
}

impl Passengers {
    pub fn new(num_passengers: f64, weight_per_passenger: f64, load_case: LoadCase) -> Self {
        Self {
            num_passengers,
            weight_per_passenger,
            load_case,
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(self.num_passengers * self.weight_per_passenger)
    }

    pub fn pos_times_weight(&self) -> Point<f64> {
        let start_of_seats = Point::new(in_to_ft(39.37 + 6. + in_to_ft(36. + 17.)), 0., 0.);
        let length_seats = 15. * in_to_ft(32.) + in_to_ft(18.);
        let length_row = length_seats / 15.;
        match self.load_case {
            LoadCase::Center => (start_of_seats + length_seats * 0.5) * self.weight().get_val(),
            LoadCase::Front => (start_of_seats + self.num_passengers / 6. * length_row) * self.weight().get_val(),
            LoadCase::Rear => (start_of_seats + length_seats - self.num_passengers / 6. * length_row) * self.weight().get_val(),
        }
    }
}

pub struct Payload {
    payload_weight: f64,
    cg: Point<f64>,
}

impl Payload {
    pub fn new(payload_weight: f64) -> Self {
        Self {
            payload_weight,
            cg: Point::new(0., 0., 0.),
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(self.payload_weight)
    }

    pub fn pos_times_weight(&self) -> Point<f64> {
        self.cg * self.weight().get_val()
    }
}

pub struct Fuel {
    fuel_weight: f64,
    cg: Point<f64>,
}

impl Fuel {
    pub fn new(fuel_weight: f64, cg: Point<f64>) -> Self {
        Self {
            fuel_weight,
            cg,
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::new(self.fuel_weight)
    }

    pub fn pos_times_weight(&self) -> Point<f64> {
        self.cg * self.weight().get_val()
    }
}
