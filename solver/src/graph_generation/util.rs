use clustering::kmeans;

use crate::{LayeredGraph, Vertex, VertexEmbedding, VertexID};

const MAX_ITER: usize = 40;

pub fn k_means_recursive(
    graph: &mut LayeredGraph,
    parent_index: usize,
    layer_index: usize,
    sources: &Vec<usize>,
    capacities: &Vec<usize>,
    sources_embeddings: &Vec<VertexEmbedding>,
) {
    if layer_index == 0 {
        for index in sources {
            let mut new_vertex = Vertex::new_empty();
            new_vertex.set_parent(Some(parent_index));
            graph.set_vertex_at_position(&VertexID::new(0, *index), new_vertex);
        }
        return;
    }

    let capacity = capacities[layer_index];

    let cluster_mappings = get_cluster_mappings(sources, capacity, sources_embeddings);

    for cluster in cluster_mappings {
        let mut new_vertex = Vertex::new_empty();
        new_vertex.set_parent(Some(parent_index));
        let vertex_id = graph.add_vertex_to_layer(layer_index, new_vertex);

        k_means_recursive(
            graph,
            vertex_id.index,
            layer_index - 1,
            &cluster,
            capacities,
            sources_embeddings,
        );
    }
}

fn get_cluster_mappings(
    sources: &Vec<usize>,
    capacity: usize,
    sources_embeddings: &Vec<VertexEmbedding>,
) -> Vec<Vec<usize>> {
    let k = if sources.len() % capacity == 0 {
        sources.len() / capacity
    } else {
        sources.len() / capacity + 1
    };

    let mut samples = vec![vec![0.0; 2]; sources.len()];

    for i in 0..sources.len() {
        let embedding = sources_embeddings[sources[i]];
        samples[i][0] = embedding.0;
        samples[i][1] = embedding.1;
    }

    let result = kmeans(k, &samples, MAX_ITER);

    let mut clusters = vec![Vec::new(); k];
    for i in 0..samples.len() {
        let cluster_index = result.membership[i];
        clusters[cluster_index].push(sources[i]);
    }

    let mut result = Vec::new();
    for i in 0..clusters.len() {
        if clusters[i].len() > capacity {
            let mut recluster = get_cluster_mappings(&clusters[i], capacity, sources_embeddings);
            result.append(&mut recluster);
        } else {
            result.push(clusters[i].clone());
        }
    }

    return result;
}
