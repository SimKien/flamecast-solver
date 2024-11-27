use std::collections::HashMap;

use super::{LayeredGraph, Vertex};

pub type VertexEmbedding = (f64, f64); // (x-coordinate, y-coordinate) of a vertex

#[derive(Debug, Clone)]
pub struct VertexEmbeddings {
    pub content: HashMap<Vertex, VertexEmbedding>,
}

impl PartialEq for VertexEmbeddings {
    fn eq(&self, other: &Self) -> bool {
        if self.content.len() != other.content.len() {
            return false;
        }

        return self.content.iter().all(|(vertex, embedding)| {
            let other_embedding = other.get(vertex).unwrap();
            let (x1, y1) = embedding;
            let (x2, y2) = other_embedding;
            return (x1 - x2).abs() < 1e-6 && (y1 - y2).abs() < 1e-6;
        });
    }
}

impl VertexEmbeddings {
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
        }
    }

    pub fn from(content: HashMap<Vertex, VertexEmbedding>) -> Self {
        Self { content }
    }

    pub fn insert(&mut self, vertex: Vertex, embedding: VertexEmbedding) {
        self.content.insert(vertex, embedding);
    }

    pub fn get(&self, vertex: &Vertex) -> Option<&VertexEmbedding> {
        self.content.get(vertex)
    }

    pub fn extend(&mut self, other: &HashMap<Vertex, VertexEmbedding>) {
        self.content.extend(other.iter());
    }
}

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
                let (x1, y1) = self.vertices_embeddings.content.get(source).unwrap();
                let (x2, y2) = self.vertices_embeddings.content.get(target).unwrap();

                let edge_len = (x1 - x2).hypot(y1 - y2);
                let flow = *edge_flows.get(&(*source, *target)).unwrap();

                cost += edge_len * (flow as f64).powf(alpha);
            }
        }

        return cost;
    }
}
