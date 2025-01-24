use chrono::TimeDelta;
use rand::{distributions::WeightedIndex, prelude::Distribution, Rng};

use crate::{
    neighborhood::{Neighbor, NeighborCost},
    EmbeddingOptions, FlamecastInstance,
};

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

    pub fn get_candidate_neighbors_cost(&mut self) -> Vec<NeighborCost> {
        return self
            .flamecast_instance
            .get_candidate_neighbors_cost(&self.neighbor_test_options);
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

    pub fn solve<T: std::io::Write>(&mut self, buf: &mut T) -> f64 {
        let initial_objective_value = self.current_objective_value;

        let start_time = chrono::Utc::now();
        let mut last_time = start_time.clone();

        if self.verbose {
            writeln!(buf, "Starting Simulated Annealing...").unwrap();
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
            let mut candidate_neighbors = self.get_candidate_neighbors_cost();

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

                let current_time = chrono::Utc::now();
                let elapsed_time = current_time.signed_duration_since(last_time);
                writeln!(
                    buf,
                    "Elapsed Time For Current Step: {}",
                    get_time_diff_pretty(elapsed_time)
                )
                .unwrap();
                last_time = current_time;
            }

            if changed {
                self.flamecast_instance
                    .accepted_neighbors
                    .push(possible_neighbor);
            }

            self.iteration += 1;
        }

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

            let end_time = chrono::Utc::now();
            let elapsed_time = end_time.signed_duration_since(start_time);
            writeln!(
                buf,
                "Total Time Needed: {}",
                get_time_diff_pretty(elapsed_time)
            )
            .unwrap();
        }

        return self.current_objective_value;
    }
}

fn get_time_diff_pretty(diff: TimeDelta) -> String {
    let num_millis = diff.num_milliseconds() % 1_000;
    let num_seconds = diff.num_seconds() % 60;
    let num_minutes = (diff.num_seconds() / 60) % 60;
    let num_hours = diff.num_seconds() / 3600;
    let result = if diff.num_nanoseconds().is_some() {
        let num_nano = diff.num_nanoseconds().unwrap() % 1_000;
        let num_micro = diff.num_microseconds().unwrap() % 1_000;
        format!(
            "{}h {}min {}s {}ms {}us {}ns",
            num_hours, num_minutes, num_seconds, num_millis, num_micro, num_nano
        )
    } else if diff.num_microseconds().is_some() {
        let num_micro = diff.num_microseconds().unwrap() % 1_000;
        format!(
            "{}h {}min {}s {}ms {}us",
            num_hours, num_minutes, num_seconds, num_millis, num_micro
        )
    } else {
        format!(
            "{}h {}min {}s {}ms",
            num_hours, num_minutes, num_seconds, num_millis
        )
    };
    return result;
}
