use serde::{Deserialize, Serialize};

use crate::Neighbor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeighborChange {
    pub neighbor: Neighbor,
    pub neighbor_cost: f64,
    pub iteration: usize,
}

impl NeighborChange {
    pub fn new(neighbor: Neighbor, neighbor_cost: f64, iteration: usize) -> Self {
        Self {
            neighbor,
            neighbor_cost,
            iteration,
        }
    }
}
