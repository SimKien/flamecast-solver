use std::{f64::INFINITY, usize::MAX};

use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

use crate::{GraphEmbedding, NeighborChange};

use super::CoolingSchedule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeDeltaSave {
    pub nanos: i32,
    pub secs: i64,
}

impl TimeDeltaSave {
    pub fn new_empty() -> Self {
        Self { nanos: 0, secs: 0 }
    }

    pub fn from_time_delta(time_delta: &TimeDelta) -> Self {
        Self {
            nanos: time_delta.num_nanoseconds().unwrap_or(0) as i32,
            secs: time_delta.num_seconds(),
        }
    }

    pub fn to_time_delta(&self) -> TimeDelta {
        TimeDelta::seconds(self.secs) + TimeDelta::nanoseconds(self.nanos as i64)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedAnnealingLogger {
    pub init_time: TimeDeltaSave,
    pub initial_solution: GraphEmbedding,
    pub final_solution: GraphEmbedding,
    pub initial_objective_value: f64,
    pub final_objective_value: f64,
    pub times_needed: Vec<TimeDeltaSave>,
    pub current_costs: Vec<f64>,
    pub current_best_costs: Vec<f64>,
    pub current_amount_nodes: Vec<usize>,
    pub accepted_neighbors: Vec<NeighborChange>,
    pub max_iterations: usize,
    pub initial_temperature: f64,
    pub cooling_schedule: CoolingSchedule,
    pub total_time: TimeDeltaSave,
}

impl SimulatedAnnealingLogger {
    pub fn new_empty() -> Self {
        Self {
            init_time: TimeDeltaSave::new_empty(),
            initial_solution: GraphEmbedding::new_empty(),
            final_solution: GraphEmbedding::new_empty(),
            initial_objective_value: INFINITY,
            final_objective_value: INFINITY,
            times_needed: Vec::new(),
            current_costs: Vec::new(),
            current_best_costs: Vec::new(),
            current_amount_nodes: Vec::new(),
            accepted_neighbors: Vec::new(),
            max_iterations: MAX,
            initial_temperature: INFINITY,
            cooling_schedule: CoolingSchedule::Exponential(0.0),
            total_time: TimeDeltaSave::new_empty(),
        }
    }

    pub fn from_init_time(init_time: TimeDeltaSave) -> Self {
        Self {
            init_time,
            initial_solution: GraphEmbedding::new_empty(),
            final_solution: GraphEmbedding::new_empty(),
            initial_objective_value: INFINITY,
            final_objective_value: INFINITY,
            times_needed: Vec::new(),
            current_costs: Vec::new(),
            current_best_costs: Vec::new(),
            current_amount_nodes: Vec::new(),
            accepted_neighbors: Vec::new(),
            max_iterations: MAX,
            initial_temperature: INFINITY,
            cooling_schedule: CoolingSchedule::Exponential(0.0),
            total_time: TimeDeltaSave::new_empty(),
        }
    }

    pub fn new(
        init_time: TimeDeltaSave,
        initial_solution: &GraphEmbedding,
        initial_costs: f64,
        max_iterations: usize,
        initial_temperature: f64,
        cooling_schedule: CoolingSchedule,
    ) -> Self {
        Self {
            init_time,
            initial_solution: initial_solution.clone(),
            final_solution: GraphEmbedding::new_empty(),
            initial_objective_value: initial_costs,
            final_objective_value: INFINITY,
            times_needed: Vec::new(),
            current_costs: Vec::new(),
            current_best_costs: Vec::new(),
            current_amount_nodes: Vec::new(),
            accepted_neighbors: Vec::new(),
            max_iterations,
            initial_temperature,
            cooling_schedule,
            total_time: TimeDeltaSave::new_empty(),
        }
    }

    pub fn log(
        &mut self,
        time_needed: TimeDelta,
        current_cost: f64,
        current_best_cost: f64,
        current_amount_nodes: usize,
    ) {
        self.times_needed
            .push(TimeDeltaSave::from_time_delta(&time_needed));
        self.current_costs.push(current_cost);
        self.current_best_costs.push(current_best_cost);
        self.current_amount_nodes.push(current_amount_nodes);
    }

    pub fn log_accepted_neighbor(&mut self, neighbor: NeighborChange) {
        self.accepted_neighbors.push(neighbor);
    }

    pub fn set_final_solution(
        &mut self,
        final_solution: &GraphEmbedding,
        final_objective_value: f64,
        total_time: TimeDelta,
    ) {
        self.final_solution = final_solution.clone();
        self.final_objective_value = final_objective_value;
        self.total_time = TimeDeltaSave::from_time_delta(&total_time);
    }
}
