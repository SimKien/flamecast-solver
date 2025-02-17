use rand::{distributions::WeightedIndex, prelude::Distribution};

use crate::{neighborhood::NeighborCost, Neighbor};

const CHOOSE_NEIGHBOR_PROBABILITY_SCALE: f64 = 10.0;

pub fn select_neighbor_from_complete(
    candidate_neighbors: &Vec<NeighborCost>,
    current_objective_value: f64,
) -> Neighbor {
    let mut weights = vec![0.0; candidate_neighbors.len()];
    candidate_neighbors
        .iter()
        .enumerate()
        .for_each(|(index, neighbor_cost)| {
            //e^(CHOOSE_NEIGHBOR_PROBABILITY_SCALE * ((current_objective_value - neighbor_cost) / current_objective_value))
            weights[index] = (CHOOSE_NEIGHBOR_PROBABILITY_SCALE
                * (1.0 - (neighbor_cost.cost / current_objective_value)))
                .exp();
        });

    let mut rng = rand::thread_rng();
    let dist = WeightedIndex::new(&weights).unwrap();
    return candidate_neighbors[dist.sample(&mut rng)].neighbor.clone();
}

pub fn select_neighbor_from_heuristical(candidate_neighbors: &Vec<NeighborCost>) -> Neighbor {
    return candidate_neighbors
        .iter()
        .reduce(|x, y| x.min(y))
        .unwrap()
        .neighbor
        .clone();
}
