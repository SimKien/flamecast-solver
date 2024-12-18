use super::LayeredGraph;

pub type VertexEmbedding = (f64, f64); // (x-coordinate, y-coordinate) of a vertex

#[derive(Debug, Clone)]
pub struct VertexEmbeddings {
    pub embeddings: Vec<Vec<VertexEmbedding>>,
}

impl VertexEmbeddings {
    pub fn new() -> Self {
        Self {
            embeddings: Vec::new(),
        }
    }

    pub fn new_with_size(layers: usize) -> Self {
        Self {
            embeddings: vec![vec![]; layers],
        }
    }

    pub fn from(embeddings: Vec<Vec<VertexEmbedding>>) -> Self {
        Self { embeddings }
    }
}

#[derive(Debug, Clone)]
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
        // calculate the cost of the embedding, assumes a valid flamecast graph
        let edge_flows = self.base_graph.calculate_edge_flows();
        let mut cost = 0.0;

        for layer in self.base_graph.layers.iter() {
            if layer.vertices[0].vertex_id.layer == self.base_graph.layers.len() - 1 {
                break;
            }

            layer.vertices.iter().for_each(|vertex| {
                let source_embedding = self.vertices_embeddings.embeddings[vertex.vertex_id.layer]
                    [vertex.vertex_id.index];
                let target_embedding = self.vertices_embeddings.embeddings
                    [vertex.vertex_id.layer + 1][vertex.parent_index.unwrap()];

                let edge_len = ((source_embedding.0 - target_embedding.0).powi(2)
                    + (source_embedding.1 - target_embedding.1).powi(2))
                .sqrt();

                cost += edge_len
                    * (edge_flows[vertex.vertex_id.layer][vertex.vertex_id.index] as f64)
                        .powf(alpha);
            });
        }

        return cost;
    }
}
