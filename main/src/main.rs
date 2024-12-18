use std::env;

use rand::Rng;
use solver::{
    combine_testing_graphs, embed_graph, generate_random_graph, get_test_graph,
    get_test_graphs_len, plot_graph, Options, SearchDepth, TestGraph, VertexEmbeddings,
};

pub const DEFAULT_NUM_NODES: usize = 200;
pub const DEFAULT_NUM_LAYERS: usize = 5;
pub const DEFAULT_ALPHA: f64 = 0.9;
pub const SEARCH_DEPTH: SearchDepth = SearchDepth::Middle;
pub const TIME_LIMIT: f64 = f64::INFINITY;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let test_graph = convert_args_to_graph(args);
    if test_graph.is_none() {
        println!("Usage: cargo run");
        println!("Usage: cargo run <test_graph_index>,<test_graph_index>,...");
        println!("Usage: cargo run <num_nodes> <num_layers> <alpha>");
        println!("When combining test graphs, the graphs must have the same number of layers.");
        return;
    }
    let test_graph = test_graph.unwrap();

    let embedded_graph = embed_graph(
        test_graph.graph,
        &test_graph.sources_drains_embeddings,
        test_graph.alpha,
        Options::new(true, SEARCH_DEPTH, TIME_LIMIT, true, false),
    );

    plot_graph("./plots/output.png", &embedded_graph, true);
}

fn convert_args_to_graph(args: Vec<String>) -> Option<TestGraph> {
    match args.len() {
        1 => {
            let graph = generate_random_graph(DEFAULT_NUM_NODES, DEFAULT_NUM_LAYERS);

            let x_delta = 1.0 / (DEFAULT_NUM_LAYERS as f64);
            let side_distance = x_delta / 2.0;

            let mut rng = rand::thread_rng();
            let mut sources_drains_embeddings = VertexEmbeddings::new_with_size(DEFAULT_NUM_LAYERS);

            for _ in graph.get_sources_indexes().iter() {
                let y = rng.gen_range(0.0..=1.0);
                sources_drains_embeddings.embeddings[0].push((side_distance, y));
            }
            for _ in graph.get_drains_indexes().iter() {
                let y = rng.gen_range(0.0..=1.0);
                sources_drains_embeddings.embeddings[DEFAULT_NUM_LAYERS - 1]
                    .push((1.0 - side_distance, y));
            }

            Some(TestGraph {
                graph,
                sources_drains_embeddings,
                alpha: DEFAULT_ALPHA,
            })
        }
        2 => {
            let test_graph_indexes = args[1]
                .split(',')
                .map(|index| index.parse::<usize>())
                .collect::<Vec<Result<usize, _>>>();

            if test_graph_indexes.iter().any(|index| index.is_err()) {
                return None;
            }
            let test_graph_indexes = test_graph_indexes
                .iter()
                .map(|index| index.clone().unwrap())
                .collect::<Vec<usize>>();
            if test_graph_indexes
                .iter()
                .any(|index| *index >= get_test_graphs_len())
            {
                return None;
            }

            let mut graph = get_test_graph(test_graph_indexes[0]);
            for index in test_graph_indexes.iter().skip(1) {
                let graph2 = get_test_graph(*index);
                let new_graph = combine_testing_graphs(&mut graph, &graph2);
                if new_graph.is_none() {
                    return None;
                }
                graph = new_graph.unwrap();
            }

            Some(graph)
        }
        4 => {
            let num_nodes = args[1].parse::<usize>();
            let num_layers = args[2].parse::<usize>();
            let alpha = args[3].parse::<f64>();
            if num_nodes.is_err() || num_layers.is_err() || alpha.is_err() {
                return None;
            }

            let num_nodes = num_nodes.unwrap();
            let num_layers = num_layers.unwrap();
            let alpha = alpha.unwrap();
            let graph = generate_random_graph(num_nodes, num_layers);

            let x_delta = 1.0 / (num_layers as f64);
            let side_distance = x_delta / 2.0;

            let mut rng = rand::thread_rng();
            let mut sources_drains_embeddings = VertexEmbeddings::new_with_size(num_layers);

            for _ in graph.get_sources_indexes().iter() {
                let y = rng.gen_range(0.0..=1.0);
                sources_drains_embeddings.embeddings[0].push((side_distance, y));
            }
            for _ in graph.get_drains_indexes().iter() {
                let y = rng.gen_range(0.0..=1.0);
                sources_drains_embeddings.embeddings[num_layers - 1].push((1.0 - side_distance, y));
            }

            Some(TestGraph {
                graph,
                sources_drains_embeddings,
                alpha,
            })
        }
        _ => None,
    }
}
