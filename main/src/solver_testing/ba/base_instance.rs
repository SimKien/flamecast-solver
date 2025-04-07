use serde::{Deserialize, Serialize};
use solver::{FlamecastTestInstance, VertexEmbedding};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlamecastBaseInstance {
    pub sources: Vec<VertexEmbedding>,
    pub drains: Vec<VertexEmbedding>,
    pub layers: usize,
    pub capacities: Vec<usize>,
}

impl FlamecastBaseInstance {
    pub fn from_test_instance(test_instance: &FlamecastTestInstance) -> Self {
        FlamecastBaseInstance {
            sources: test_instance.sources_drains_embeddings.embeddings[0].clone(),
            drains: test_instance.sources_drains_embeddings.embeddings
                [test_instance.num_layers - 1]
                .clone(),
            layers: test_instance.num_layers,
            capacities: test_instance.capacities.clone(),
        }
    }

    pub fn from_file(file_path: &String) -> Self {
        let content: String = std::fs::read_to_string(file_path).expect("Failed to read file");
        serde_json::from_str(&content).expect("Failed to parse JSON")
    }

    pub fn to_file(&self, file_path: &String) {
        let content = serde_json::to_string_pretty(self).expect("Failed to serialize to JSON");
        std::fs::write(file_path, content).expect("Failed to write file");
    }
}
