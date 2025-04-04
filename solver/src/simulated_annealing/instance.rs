use rand::Rng;

use crate::{
    neighborhood::{Neighbor, NeighborCost},
    EmbeddingOptions, FlamecastInstance, NeighborChange, Stopwatch,
};

use super::{
    neighborhood_change_probability, select_neighbor_from_complete,
    select_neighbor_from_heuristical, CoolingSchedule, NeighborSearchOption, OptimizationOptions,
    SimulatedAnnealingLogger,
};

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
        let logger = SimulatedAnnealingLogger::new(
            &flamecast_instance.solution_state.current_solution,
            current_objective_value,
            optimization_options.max_iterations,
            optimization_options.initial_temperature,
            optimization_options.cooling_schedule.clone(),
        );
        flamecast_instance.logger = logger;
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
        return match self.neighbor_search_option {
            NeighborSearchOption::CompleteEmbedding => self
                .flamecast_instance
                .get_all_candidate_neighbors_cost(&self.neighbor_test_options),
            NeighborSearchOption::CompleteHeuristical => self
                .flamecast_instance
                .get_heuristical_candidate_neighbors_cost(&self.neighbor_test_options),
        };
    }

    pub fn choose_candidate_neighbor(
        &mut self,
        candidate_neighbors: &Vec<NeighborCost>,
    ) -> Neighbor {
        return match self.neighbor_search_option {
            NeighborSearchOption::CompleteEmbedding => {
                select_neighbor_from_complete(candidate_neighbors, self.current_objective_value)
            }
            NeighborSearchOption::CompleteHeuristical => {
                select_neighbor_from_heuristical(candidate_neighbors)
            }
        };
    }

    pub fn neighbor_change(&mut self, neighbor_cost: f64, neighbor: &Neighbor) {
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
                return;
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
        let neighbor_change = NeighborChange::new(
            neighbor.clone(),
            self.current_objective_value,
            self.iteration,
        );
        solution_state
            .accepted_neighbors
            .push(neighbor_change.clone());
        self.flamecast_instance
            .logger
            .log_accepted_neighbor(neighbor_change.clone());
        solution_state
            .best_iteration
            .update(self.current_objective_value, self.iteration);
    }

    pub fn solve(&mut self) {
        let initial_objective_value = self.current_objective_value;

        let start_watch = Stopwatch::new();

        if self.verbose {
            println!("Simulated Annealing Started");
            println!("Initial Objective Value: {}", initial_objective_value);
        }

        while self.iteration < self.max_iterations {
            // details for logging
            let current_iteration_watch = Stopwatch::new();

            // if temperature gets negative, stop iteration
            if self
                .cooling_schedule
                .get_temperature(self.initial_temperature, self.iteration)
                <= 0.0
            {
                break;
            }

            // calculate possible neighbors with corresponding costs
            let mut candidate_neighbors = self.get_candidate_neighbors();
            if candidate_neighbors.is_empty() {
                break;
            }

            // select neighbor of candidate list
            let possible_neighbor = self.choose_candidate_neighbor(&mut candidate_neighbors);

            // calculate cost of the selected neighbor
            let neighbor_cost = self
                .flamecast_instance
                .get_neighbor_cost(&possible_neighbor, &self.neighbor_cost_options);

            // perform the neighbor change
            self.neighbor_change(neighbor_cost, &possible_neighbor);

            // log the current state of the algorithm
            let time_needed = current_iteration_watch.elapsed_pretty();
            self.flamecast_instance.logger.log(
                time_needed.clone(),
                self.current_objective_value,
                self.flamecast_instance
                    .solution_state
                    .best_iteration
                    .best_value,
                self.flamecast_instance
                    .solution_state
                    .current_solution
                    .base_graph
                    .get_number_of_vertices(),
            );

            if self.verbose {
                println!(
                    "Iteration: {} - Current Objective Value: {} - Best Objective Value: {} - Time Needed: {}",
                    self.iteration,
                    self.current_objective_value,
                    self.flamecast_instance
                        .solution_state
                        .best_iteration
                        .best_value,
                    time_needed
                );
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

        self.flamecast_instance.logger.set_final_solution(
            &self.flamecast_instance.solution_state.current_solution,
            self.current_objective_value,
            start_watch.elapsed_pretty(),
        );

        if self.verbose {
            println!("Simulated Annealing Finished");
        }
    }
}
