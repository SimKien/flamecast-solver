use std::collections::HashMap;

use super::{DirectedGraph, Vertex};

pub type EmbeddedVertex = (f32, f32); // (x-coordinate, y-coordinate) of a vertex

pub type VertexEmbeddings = HashMap<Vertex, EmbeddedVertex>;

pub struct GraphEmbedding {
    pub base_graph: DirectedGraph,
    pub vertices_embeddings: VertexEmbeddings,
}

impl GraphEmbedding {
    pub fn new(base_graph: DirectedGraph, vertices_embeddings: VertexEmbeddings) -> Self {
        Self {
            base_graph,
            vertices_embeddings,
        }
    }

    pub fn calculate_costs(&self, alpha: f32) -> f32 {
        // evaluate the cost of the embedding
        let edge_flows = self.base_graph.calculate_edge_flows();
        let mut cost = 0.0;

        for edge in self.base_graph.edges.iter() {
            let (x1, y1) = self.vertices_embeddings.get(&edge.0).unwrap();
            let (x2, y2) = self.vertices_embeddings.get(&edge.1).unwrap();

            let edge_len = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
            let flow = *edge_flows.get(&edge).unwrap();

            cost += edge_len * (flow as f32).powf(alpha);
        }

        return cost;
    }
}
