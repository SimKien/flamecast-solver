#[cfg(test)]
use crate::{neighborhood::Neighbor, LayeredGraph, VertexID};

#[cfg(test)]
pub fn calculate_all_neighbors(graph: &LayeredGraph) -> Vec<Neighbor> {
    let mut result = get_all_recablings(graph);
    result.append(&mut &mut get_all_swaps(graph));
    result.append(&mut get_all_merges(graph));
    result.append(&mut get_all_splits(graph));
    return result;
}

#[cfg(test)]
pub fn get_all_recablings(graph: &LayeredGraph) -> Vec<Neighbor> {
    let mut possible_recablings = Vec::new();

    let num_layers = graph.layers.len();
    for layer in 0..num_layers - 1 {
        for node in 0..graph.layers[layer].vertices.len() {
            let node_id = VertexID::new(layer, node);

            for target_node in 0..graph.layers[layer + 1].vertices.len() {
                let target_node_id = VertexID::new(layer + 1, target_node);

                if graph.get_parent(&node_id).unwrap() != target_node_id {
                    possible_recablings.push(Neighbor::Recable(node_id.clone(), target_node_id));
                }
            }
        }
    }

    return possible_recablings;
}

#[cfg(test)]
pub fn get_all_swaps(graph: &LayeredGraph) -> Vec<Neighbor> {
    let mut possible_swaps = Vec::new();

    let num_layers = graph.layers.len();
    for layer in 0..num_layers - 1 {
        for node1_index in 0..graph.layers[layer].vertices.len() {
            let node1_id = VertexID::new(layer, node1_index);

            for node2_index in (node1_index + 1)..graph.layers[layer].vertices.len() {
                let node2_id = VertexID::new(layer, node2_index);

                if graph.get_parent(&node1_id).unwrap() != graph.get_parent(&node2_id).unwrap() {
                    possible_swaps.push(Neighbor::Swap(node1_id.clone(), node2_id));
                }
            }
        }
    }

    return possible_swaps;
}

#[cfg(test)]
pub fn get_all_merges(graph: &LayeredGraph) -> Vec<Neighbor> {
    let mut possible_merges = Vec::new();

    let num_layers = graph.layers.len();
    for layer in 1..num_layers - 1 {
        for node1_index in 0..graph.layers[layer].vertices.len() {
            let node1_id = VertexID::new(layer, node1_index);

            for node2_index in (node1_index + 1)..graph.layers[layer].vertices.len() {
                let node2_id = VertexID::new(layer, node2_index);

                if graph.get_parent(&node1_id).unwrap() == graph.get_parent(&node2_id).unwrap() {
                    possible_merges.push(Neighbor::Merge(node1_id.clone(), node2_id));
                }
            }
        }
    }

    return possible_merges;
}

#[cfg(test)]
fn get_all_splits(graph: &LayeredGraph) -> Vec<Neighbor> {
    let mut possible_splits = Vec::new();

    let num_layers = graph.layers.len();
    for layer in 1..num_layers - 1 {
        for parent in 0..graph.layers[layer].vertices.len() {
            let parent_id = VertexID::new(layer, parent);

            let mut children = graph.get_children(&parent_id).unwrap();
            if children.len() > 1 {
                let children_parent2 = vec![children.pop().unwrap()];
                possible_splits.push(Neighbor::Split(children_parent2));
            }
        }
    }

    return possible_splits;
}
