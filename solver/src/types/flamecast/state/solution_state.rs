use serde::{Deserialize, Serialize};

use crate::GraphEmbedding;

use super::{BestIterationValue, NeighborChange};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolutionState {
    pub initial_solution: GraphEmbedding,
    pub current_solution: GraphEmbedding,
    pub accepted_neighbors: Vec<NeighborChange>,
    pub best_iteration: BestIterationValue,
}

impl SolutionState {
    pub fn new(current_solution: GraphEmbedding) -> Self {
        Self {
            initial_solution: current_solution.clone(),
            current_solution,
            accepted_neighbors: Vec::new(),
            best_iteration: BestIterationValue::new(),
        }
    }
}
