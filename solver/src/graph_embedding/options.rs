#[derive(Debug, Clone, Copy)]
pub enum SearchDepth {
    Shallow = 100,
    Middle = 200,
    Deep = 500,
    VeryDeep = 1000,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub print_embedding_infos: bool,
    pub search_depth: SearchDepth,
    pub show_calculated_actual_edge_length_diff: bool,
    pub verbose: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            print_embedding_infos: true,
            search_depth: SearchDepth::Middle,
            show_calculated_actual_edge_length_diff: false,
            verbose: false,
        }
    }
}

impl Options {
    pub fn new(
        print_embedding_infos: bool,
        search_depth: SearchDepth,
        show_calculated_actual_edge_length_diff: bool,
        verbose: bool,
    ) -> Self {
        Options {
            print_embedding_infos,
            search_depth,
            show_calculated_actual_edge_length_diff,
            verbose,
        }
    }
}
