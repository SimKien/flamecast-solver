#[derive(Debug, Clone, Copy)]
pub enum CoolingSchedule {
    Exponential(f64),
    Linear(f64),
    Fast(f64),
    Logarithmic(f64),
}

impl CoolingSchedule {
    pub fn get_temperature(&self, initial_temperature: f64, iteration: usize) -> f64 {
        match self {
            CoolingSchedule::Exponential(alpha) => {
                initial_temperature * alpha.powi(iteration as i32)
            }
            CoolingSchedule::Linear(alpha) => initial_temperature - alpha * iteration as f64,
            CoolingSchedule::Fast(alpha) => initial_temperature / (1.0 + alpha * iteration as f64),
            CoolingSchedule::Logarithmic(alpha) => {
                initial_temperature / (1.0 + alpha * (1.0 + iteration as f64).ln())
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CoolingSchedule::Exponential(alpha) => format!("Exponential({})", alpha),
            CoolingSchedule::Linear(alpha) => format!("Linear({})", alpha),
            CoolingSchedule::Fast(alpha) => format!("Fast({})", alpha),
            CoolingSchedule::Logarithmic(alpha) => format!("Logarithmic({})", alpha),
        }
    }
}
