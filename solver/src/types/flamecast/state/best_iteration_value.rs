use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestIterationValue {
    pub best_value: f64,
    pub iteration: usize,
}

impl BestIterationValue {
    pub fn new() -> Self {
        Self {
            best_value: f64::MAX,
            iteration: 0,
        }
    }

    pub fn update(&mut self, value: f64, iteration: usize) {
        if value < self.best_value {
            self.best_value = value;
            self.iteration = iteration;
        }
    }
}
