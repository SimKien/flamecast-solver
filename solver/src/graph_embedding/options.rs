#[derive(Debug, Clone, Copy)]
pub enum SearchDepth {
    Shallow = 100,
    Middle = 200,
    Deep = 500,
    VeryDeep = 1000,
}

#[derive(Debug, Clone)]
pub struct EmbeddingOptions {
    pub print_embedding_infos: bool,
    pub search_depth: SearchDepth,
    pub time_limit: f64,
    pub show_calculated_actual_edge_length_diff: bool,
    pub verbose: bool,
}

impl Default for EmbeddingOptions {
    fn default() -> Self {
        EmbeddingOptions {
            print_embedding_infos: true,
            search_depth: SearchDepth::Middle,
            time_limit: f64::INFINITY,
            show_calculated_actual_edge_length_diff: false,
            verbose: false,
        }
    }
}

impl EmbeddingOptions {
    pub fn new(
        print_embedding_infos: bool,
        search_depth: SearchDepth,
        time_limit: f64,
        show_calculated_actual_edge_length_diff: bool,
        verbose: bool,
    ) -> Self {
        EmbeddingOptions {
            print_embedding_infos,
            search_depth,
            time_limit,
            show_calculated_actual_edge_length_diff,
            verbose,
        }
    }
}
