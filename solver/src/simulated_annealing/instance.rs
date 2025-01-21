use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};

use crate::{neighborhood::Neighbor, EmbeddingOptions, FlamecastInstance};

use super::{neighborhood_change_probability, CoolingSchedule, OptimizationOptions};

const CHOOSE_NEIGHBOR_PROBABILITY_SCALE: f64 = 10.0;

#[derive(Debug)]
pub struct SimulatedAnnealing<'a> {
    pub flamecast_instance: &'a mut FlamecastInstance,
    pub current_objective_value: f64,
    pub cooling_schedule: CoolingSchedule,
    pub initial_temperature: f64,
    pub iteration: usize,
    pub max_iterations: usize,
    pub verbose: bool,
    pub neighbor_test_options: EmbeddingOptions,
    pub neighbor_cost_options: EmbeddingOptions,
    pub final_cost_options: EmbeddingOptions,
}

impl<'a> SimulatedAnnealing<'a> {
    pub fn from_flamecast_instance(
        flamecast_instance: &'a mut FlamecastInstance,
        optimization_options: OptimizationOptions,
    ) -> Self {
        let current_objective_value = flamecast_instance
            .current_solution
            .calculate_costs(flamecast_instance.alpha);
        Self {
            flamecast_instance,
            current_objective_value,
            cooling_schedule: optimization_options.cooling_schedule,
            initial_temperature: optimization_options.initial_temperature,
            iteration: 0,
            max_iterations: optimization_options.max_iterations,
            verbose: optimization_options.verbose,
            neighbor_test_options: optimization_options.neighbor_test_options,
            neighbor_cost_options: optimization_options.neighbor_cost_options,
            final_cost_options: optimization_options.final_cost_options,
        }
    }

    pub fn get_candidate_neighbor(&mut self) -> Neighbor {
        let mut candidate_neighbors = self
            .flamecast_instance
            .get_candidate_neighbors_cost(&self.neighbor_test_options);

        let current_objective_value = self.current_objective_value;
        candidate_neighbors.iter_mut().for_each(|neighbor_cost| {
            //e^(CHOOSE_NEIGHBOR_PROBABILITY_SCALE * ((current_objective_value - neighbor_cost) / current_objective_value))
            neighbor_cost.cost = (CHOOSE_NEIGHBOR_PROBABILITY_SCALE
                * (1.0 - (neighbor_cost.cost / current_objective_value)))
                .exp();
        });

        let mut rng = rand::thread_rng();
        let dist = WeightedIndex::new(
            &candidate_neighbors
                .iter()
                .map(|neighbor_cost| neighbor_cost.cost)
                .collect::<Vec<f64>>(),
        )
        .unwrap();
        return candidate_neighbors[dist.sample(&mut rng)].neighbor.clone();
    }

    pub fn neighbor_change(&mut self, neighbor_cost: f64, neighbor: &Neighbor) -> bool {
        if neighbor_cost >= self.current_objective_value {
            // accept the neighbor with a probability
            // based on the current temperature
            // and the difference between the current
            // and the neighbor's objective function value
            let current_temperature = self
                .cooling_schedule
                .get_temperature(self.initial_temperature, self.iteration);

            let acceptance_probability = neighborhood_change_probability(
                self.current_objective_value,
                neighbor_cost,
                current_temperature,
            );

            let mut rng = rand::thread_rng();
            let random_number: f64 = rng.gen_range(0.0..=1.0);
            if random_number >= acceptance_probability {
                // The current embedding is the embedding of the not accepted neighbor so the embedding must be reloaded when neighbor is not accepted
                self.flamecast_instance
                    .embed_current_solution(&self.neighbor_cost_options);
                return false;
            }
        }
        self.flamecast_instance
            .current_solution
            .base_graph
            .apply_neighbor_change(neighbor);
        self.current_objective_value = self
            .flamecast_instance
            .calculate_objective_function_value(&self.neighbor_cost_options);

        return true;
    }

    pub fn solve(&mut self) -> f64 {
        while self.iteration < self.max_iterations {
            let possible_neighbor = self.get_candidate_neighbor();
            let neighbor_cost = self
                .flamecast_instance
                .get_neighbor_cost(&possible_neighbor, &self.neighbor_cost_options);

            let changed = self.neighbor_change(neighbor_cost, &possible_neighbor);

            if self.verbose {
                println!("Iteraion: {}", self.iteration);
                println!("Possible Neighbor: {}", possible_neighbor.to_string());
                println!("Current Objective Value: {}", self.current_objective_value);
                println!("Neighbor Objective Value: {}", neighbor_cost);
                let temperature = self
                    .cooling_schedule
                    .get_temperature(self.initial_temperature, self.iteration);
                println!("Temperature: {}", temperature);
                let acceptance_probability = if neighbor_cost < self.current_objective_value {
                    1.0
                } else {
                    neighborhood_change_probability(
                        self.current_objective_value,
                        neighbor_cost,
                        temperature,
                    )
                };
                println!("Acceptance Probability: {}", acceptance_probability);

                if changed {
                    println!("Solution Changed With Neighbor");
                } else {
                    println!("Solution Not Changed With Neighbor");
                }
            }

            self.iteration += 1;
        }

        self.current_objective_value = self
            .flamecast_instance
            .calculate_objective_function_value(&self.final_cost_options);

        return self.current_objective_value;
    }
}
