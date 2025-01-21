mod embedding_test_graphs;
mod test_graph_embedding;

pub use embedding_test_graphs::*;

#[cfg(test)]
pub use test_graph_embedding::compare_with_generalized_weiszfeld;