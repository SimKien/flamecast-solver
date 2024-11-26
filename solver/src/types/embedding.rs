use std::collections::HashMap;

use super::{LayeredGraph, Vertex};

pub type VertexEmbedding = (f64, f64); // (x-coordinate, y-coordinate) of a vertex

pub type VertexEmbeddings = HashMap<Vertex, VertexEmbedding>;

pub struct GraphEmbedding {
    pub base_graph: LayeredGraph,
    pub vertices_embeddings: VertexEmbeddings,
}

impl GraphEmbedding {
    pub fn new(base_graph: LayeredGraph, vertices_embeddings: VertexEmbeddings) -> Self {
        Self {
            base_graph,
            vertices_embeddings,
        }
    }

    pub fn calculate_costs(&self, alpha: f64) -> f64 {
        // calculate the cost of the embedding
        let edge_flows = self.base_graph.calculate_edge_flows();
        let mut cost = 0.0;

        for layer in &self.base_graph.layers {
            for (source, target) in &layer.edges {
                let (x1, y1) = self.vertices_embeddings.get(source).unwrap();
                let (x2, y2) = self.vertices_embeddings.get(target).unwrap();

                let edge_len = (x1 - x2).hypot(y1 - y2);
                let flow = *edge_flows.get(&(*source, *target)).unwrap();

                cost += edge_len * (flow as f64).powf(alpha);
            }
        }

        return cost;
    }
}
