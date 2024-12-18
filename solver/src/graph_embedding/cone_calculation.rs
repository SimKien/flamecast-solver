use clarabel::solver::SupportedConeT::{self, SecondOrderConeT};

pub fn calculate_cones(number_of_edges: usize) -> Vec<SupportedConeT<f64>> {
    // calculate cones for clarabel
    return vec![SecondOrderConeT(3); number_of_edges];
}
