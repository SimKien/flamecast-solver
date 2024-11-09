use clarabel::solver::SupportedConeT::{self, SecondOrderConeT, ZeroConeT};

pub fn calculate_cones(
    number_of_endpoints: usize,
    number_of_edges: usize,
) -> Vec<SupportedConeT<f64>> {
    // calculate cones for clarabel
    let mut cones = vec![ZeroConeT(2 * number_of_endpoints)];
    for _ in 0..number_of_edges {
        cones.push(SecondOrderConeT(3));
    }
    return cones;
}
