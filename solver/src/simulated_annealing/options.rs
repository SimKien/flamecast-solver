use crate::{CoolingSchedule, EmbeddingOptions, SearchDepth};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeighborSearchOption {
    CompleteEmbedding,
    CompleteHeuristical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitialSolutionFunction {
    Random,
    Matching,
    LowConnectivity,
}

impl InitialSolutionFunction {
    pub fn iterate() -> impl Iterator<Item = Self> {
        vec![
            InitialSolutionFunction::Random,
            InitialSolutionFunction::Matching,
        ]
        .into_iter()
    }

    pub fn to_string(&self) -> String {
        match self {
            InitialSolutionFunction::Random => "random".to_string(),
            InitialSolutionFunction::Matching => "matching".to_string(),
            InitialSolutionFunction::LowConnectivity => "low connectivity".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptimizationOptions {
    pub cooling_schedule: CoolingSchedule,
    pub initial_temperature: f64,
    pub neighbor_search_option: NeighborSearchOption,
    pub max_iterations: usize,
    pub number_random_vertices: usize,
    pub verbose: bool,
    pub neighbor_test_options: EmbeddingOptions,
    pub neighbor_cost_options: EmbeddingOptions,
    pub final_cost_options: EmbeddingOptions,
}

impl OptimizationOptions {
    pub fn new(
        cooling_schedule: CoolingSchedule,
        initial_temperature: f64,
        neighbor_search_option: NeighborSearchOption,
        max_iterations: usize,
        number_random_vertices: usize,
        verbose: bool,
        neighbor_test_options: EmbeddingOptions,
        neighbor_cost_options: EmbeddingOptions,
        final_cost_options: EmbeddingOptions,
    ) -> Self {
        Self {
            cooling_schedule,
            initial_temperature,
            neighbor_search_option,
            max_iterations,
            number_random_vertices,
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
            neighbor_search_option: NeighborSearchOption::CompleteEmbedding,
            max_iterations: 80,
            number_random_vertices: 2,
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
