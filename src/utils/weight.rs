use std::ops;

pub struct Weight {
    val: f64,
}

impl Weight {
    pub fn new(val: f64) -> Weight {
        // if val < 0. { panic!("negative weight"); }
        Weight { val }
    }

    pub fn get_val(self) -> f64 {
        self.val
    }
}

impl ops::Add<Weight> for Weight {
    type Output = Weight;

    fn add(self, rhs: Weight) -> Self::Output {
        Weight::new(self.val + rhs.val)
    }
}

