use crate::{CoolingSchedule, EmbeddingOptions, SearchDepth};

#[derive(Debug, Clone)]
pub struct OptimizationOptions {
    pub cooling_schedule: CoolingSchedule,
    pub initial_temperature: f64,
    pub max_iterations: usize,
    pub verbose: bool,
    pub neighbor_test_options: EmbeddingOptions,
    pub neighbor_cost_options: EmbeddingOptions,
    pub final_cost_options: EmbeddingOptions,
}

impl OptimizationOptions {
    pub fn new(
        cooling_schedule: CoolingSchedule,
        initial_temperature: f64,
        max_iterations: usize,
        verbose: bool,
        neighbor_test_options: EmbeddingOptions,
        neighbor_cost_options: EmbeddingOptions,
        final_cost_options: EmbeddingOptions,
    ) -> Self {
        Self {
            cooling_schedule,
            initial_temperature,
            max_iterations,
            verbose,
            neighbor_test_options,
            neighbor_cost_options,
            final_cost_options,
        }
    }

    pub fn default() -> Self {
        Self {
            cooling_schedule: CoolingSchedule::Exponential(0.8),
            initial_temperature: 50.0,
            max_iterations: 80,
            verbose: true,
            neighbor_test_options: EmbeddingOptions::new(
                false,
                SearchDepth::Middle,
                f64::INFINITY,
                false,
                false,
            ),
            neighbor_cost_options: EmbeddingOptions::new(
                false,
                SearchDepth::Middle,
                f64::INFINITY,
                false,
                false,
            ),
            final_cost_options: EmbeddingOptions::new(
                false,
                SearchDepth::Middle,
                f64::INFINITY,
                false,
                false,
            ),
        }
    }
}
