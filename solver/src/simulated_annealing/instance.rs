use std::usize;

use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};

use crate::{
    neighborhood::{Neighbor, NeighborCost},
    EmbeddingOptions, FlamecastInstance, NeighborChange, Stopwatch,
};

use super::{
    neighborhood_change_probability, CoolingSchedule, NeighborSearchOption, OptimizationOptions,
};

const CHOOSE_NEIGHBOR_PROBABILITY_SCALE: f64 = 10.0;

#[derive(Debug)]
pub struct SimulatedAnnealing<'a> {
    pub flamecast_instance: &'a mut FlamecastInstance,
    pub current_objective_value: f64,
    pub cooling_schedule: CoolingSchedule,
    pub initial_temperature: f64,
    pub neighbor_search_option: NeighborSearchOption,
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
            .solution_state
            .current_solution
            .calculate_costs(flamecast_instance.alpha);
        Self {
            flamecast_instance,
            current_objective_value,
            cooling_schedule: optimization_options.cooling_schedule,
            initial_temperature: optimization_options.initial_temperature,
            neighbor_search_option: optimization_options.neighbor_search_option,
            iteration: 0,
            max_iterations: optimization_options.max_iterations,
            verbose: optimization_options.verbose,
            neighbor_test_options: optimization_options.neighbor_test_options,
            neighbor_cost_options: optimization_options.neighbor_cost_options,
            final_cost_options: optimization_options.final_cost_options,
        }
    }

    pub fn get_candidate_neighbors(&mut self) -> Vec<NeighborCost> {
        return self
            .flamecast_instance
            .get_all_candidate_neighbors_cost(&self.neighbor_test_options);
    }

    pub fn choose_candidate_neighbor(
        &mut self,
        candidate_neighbors: &mut Vec<NeighborCost>,
    ) -> Neighbor {
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
                //Re-embedding is only needed in complete Embedding search
                if self.neighbor_search_option == NeighborSearchOption::CompleteEmbedding {
                    // The current embedding is the embedding of the not accepted neighbor so the embedding must be reloaded when neighbor is not accepted
                    self.flamecast_instance
                        .embed_current_solution(&self.neighbor_cost_options);
                }
                return false;
            }
        }

        // Update solution state if neighbor is accepted
        self.flamecast_instance
            .solution_state
            .current_solution
            .base_graph
            .apply_neighbor_change(neighbor);
        self.current_objective_value = self
            .flamecast_instance
            .calculate_objective_function_value(&self.neighbor_cost_options);

        let solution_state = &mut self.flamecast_instance.solution_state;
        solution_state.accepted_neighbors.push(NeighborChange::new(
            neighbor.clone(),
            self.current_objective_value,
            self.iteration,
        ));
        solution_state
            .best_iteration
            .update(self.current_objective_value, self.iteration);

        return true;
    }

    pub fn solve<T: std::io::Write>(&mut self, buf: &mut T) -> f64 {
        let initial_objective_value = self.current_objective_value;

        let start_watch = Stopwatch::new();
        let mut current_watch = Stopwatch::new();

        if self.verbose {
            writeln!(buf, "Starting Simulated Annealing...").unwrap();
            writeln!(buf, "Started At: {}", chrono::Utc::now().to_string()).unwrap();
            writeln!(buf, "Initial Objective Value: {}", initial_objective_value).unwrap();
            writeln!(buf, "Initial Temperature: {}", self.initial_temperature).unwrap();
            writeln!(
                buf,
                "Cooling Schedule: {}",
                self.cooling_schedule.to_string()
            )
            .unwrap();
            writeln!(buf, "Max Iterations: {}", self.max_iterations).unwrap();
        }

        while self.iteration < self.max_iterations {
            //TODO: improve neighbor choose algorithm
            let mut candidate_neighbors = self.get_candidate_neighbors();
            let possible_neighbor = self.choose_candidate_neighbor(&mut candidate_neighbors);

            let neighbor_cost = self
                .flamecast_instance
                .get_neighbor_cost(&possible_neighbor, &self.neighbor_cost_options);
            let changed = self.neighbor_change(neighbor_cost, &possible_neighbor);

            if self.verbose {
                writeln!(buf, "Iteration: {}", self.iteration).unwrap();
                writeln!(
                    buf,
                    "Number of Candidate Neighbors: {}",
                    candidate_neighbors.len()
                )
                .unwrap();
                writeln!(
                    buf,
                    "Chosen candidate Neighbor: {}",
                    possible_neighbor.to_string()
                )
                .unwrap();
                writeln!(
                    buf,
                    "Current Objective Value: {}",
                    self.current_objective_value
                )
                .unwrap();
                writeln!(buf, "Neighbor Objective Value: {}", neighbor_cost).unwrap();
                let temperature = self
                    .cooling_schedule
                    .get_temperature(self.initial_temperature, self.iteration);
                writeln!(buf, "Temperature: {}", temperature).unwrap();
                let acceptance_probability = if neighbor_cost < self.current_objective_value {
                    1.0
                } else {
                    neighborhood_change_probability(
                        self.current_objective_value,
                        neighbor_cost,
                        temperature,
                    )
                };
                writeln!(buf, "Acceptance Probability: {}", acceptance_probability).unwrap();

                if changed {
                    writeln!(buf, "Solution Changed With Neighbor").unwrap();
                } else {
                    writeln!(buf, "Solution Not Changed With Neighbor").unwrap();
                }

                writeln!(
                    buf,
                    "Elapsed Time For Current Step: {}",
                    current_watch.elapsed_pretty()
                )
                .unwrap();
                current_watch.restart();
            }

            self.iteration += 1;
        }

        let current_objective_value = self.current_objective_value;
        let best_objective_value = self
            .flamecast_instance
            .solution_state
            .best_iteration
            .best_value;

        if initial_objective_value < best_objective_value {
            self.flamecast_instance.solution_state.current_solution = self
                .flamecast_instance
                .solution_state
                .initial_solution
                .clone();
            self.flamecast_instance
                .solution_state
                .best_iteration
                .update(initial_objective_value, usize::MAX);
        } else if best_objective_value < current_objective_value {
            let mut best_solution = self
                .flamecast_instance
                .solution_state
                .initial_solution
                .clone();
            for neighbor_change in self
                .flamecast_instance
                .solution_state
                .accepted_neighbors
                .iter()
            {
                best_solution
                    .base_graph
                    .apply_neighbor_change(&neighbor_change.neighbor);

                if neighbor_change.iteration
                    == self
                        .flamecast_instance
                        .solution_state
                        .best_iteration
                        .iteration
                {
                    break;
                }
            }
            self.flamecast_instance.solution_state.current_solution = best_solution;
        }

        // Calculate embedding and objective function value with the final options
        self.current_objective_value = self
            .flamecast_instance
            .calculate_objective_function_value(&self.final_cost_options);

        if self.verbose {
            writeln!(buf, "Simulated Annealing Finished").unwrap();
            writeln!(buf, "Initial Objective Value: {}", initial_objective_value).unwrap();
            writeln!(
                buf,
                "Final Objective Value: {}",
                self.current_objective_value
            )
            .unwrap();
            writeln!(
                buf,
                "Final Temperature: {}",
                self.cooling_schedule
                    .get_temperature(self.initial_temperature, self.iteration)
            )
            .unwrap();
            writeln!(
                buf,
                "Solution Quality Change: {}",
                self.current_objective_value - initial_objective_value
            )
            .unwrap();

            let capacities = &self.flamecast_instance.capacities;
            let number_of_sources = self.flamecast_instance.get_number_of_sources();
            let number_of_drains = self.flamecast_instance.get_number_of_drains();
            let num_layers = self.flamecast_instance.num_layers;
            writeln!(
                buf,
                "Final Solution Valid: {}",
                self.flamecast_instance
                    .solution_state
                    .current_solution
                    .base_graph
                    .is_valid_flamecast_topology_check_all(
                        capacities,
                        number_of_sources,
                        number_of_drains,
                        num_layers
                    )
            )
            .unwrap();

            writeln!(buf, "Total Time Needed: {}", start_watch.elapsed_pretty()).unwrap();
        }

        return self.current_objective_value;
    }
}
