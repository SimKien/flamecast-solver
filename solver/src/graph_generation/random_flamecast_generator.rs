use std::collections::HashMap;

use rand::Rng;

use crate::LayeredGraph;

pub fn generate_random_flamecast_graph(
    num_layers: usize,
    capacities: Vec<usize>,
    sources_size: usize,
    drains_size: usize,
) -> LayeredGraph {
    let sources: Vec<usize> = (0..sources_size).collect();
    let drains: Vec<usize> = (sources_size..sources_size + drains_size).collect();

    let mut graph = LayeredGraph::new_empty(); //LayeredGraph::from_sources_drains(sources.clone(), drains.clone(), num_layers);

    let rnd = &mut rand::thread_rng();
    let mut available_drains: Vec<usize> = drains.clone();
    let mut drain_sources_mappings: HashMap<usize, Vec<usize>> = HashMap::new();

    //TODO: Everx drain should have at least one source

    for source in sources.iter() {
        let drain_index = rnd.gen_range(0..available_drains.len());
        drain_sources_mappings
            .entry(available_drains[drain_index])
            .or_default()
            .push(*source);
        if drain_sources_mappings
            .get(&available_drains[drain_index])
            .unwrap()
            .len()
            >= capacities[num_layers - 1]
        {
            available_drains.remove(drain_index);
        }
    }

    for drain in drains.iter() {
        let corresponding_sources = drain_sources_mappings.get(drain).unwrap();

        //TODO find feasible random path
    }

    return graph;
}
