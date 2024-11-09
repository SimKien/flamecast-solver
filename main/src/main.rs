mod test_graphs;

use std::env;

use rand::Rng;
use solver::{
    embed_graph, generate_random_graph, get_layers, plot_graph, SearchDepth, VertexEmbeddings,
};
use test_graphs::{TestGraph, TESTGRAPHS};

pub const NUM_NODES: usize = 200;
pub const NUM_LAYERS: usize = 5;
pub const ALPHA: f64 = 0.0;
pub const SEARCH_DEPTH: SearchDepth = SearchDepth::Middle;

fn main() {
    //TODO: add comparison of d_i and the actual lengths of the edges

    let args = env::args().collect::<Vec<String>>();

    let test_graph = convert_args_to_graph(args);
    if test_graph.is_none() {
        println!("Usage: cargo run");
        println!("Usage: cargo run <test_graph_index>");
        println!("Usage: cargo run <num_nodes> <num_layers>");
        return;
    }
    let test_graph = test_graph.unwrap();

    let x_delta = 1.0 / ((test_graph.num_layers) as f64);
    let side_distance = x_delta / 2.0;

    let mut rng = rand::thread_rng();
    let mut sources_embeddings = VertexEmbeddings::new();
    let mut drains_embeddings = VertexEmbeddings::new();

    let layers = get_layers(&test_graph.graph);
    for source in layers[0].iter() {
        let y = rng.gen_range(0.0..=1.0);
        sources_embeddings.insert(*source, (side_distance, y));
    }
    for drain in layers[test_graph.num_layers - 1].iter() {
        let y = rng.gen_range(0.0..=1.0);
        drains_embeddings.insert(*drain, (1.0 - side_distance, y));
    }

    let embedded_graph = embed_graph(
        test_graph.graph,
        ALPHA,
        &sources_embeddings,
        &drains_embeddings,
        SEARCH_DEPTH,
    );

    plot_graph(&embedded_graph, true, true);
}

fn convert_args_to_graph(args: Vec<String>) -> Option<TestGraph> {
    match args.len() {
        1 => {
            let graph = generate_random_graph(NUM_NODES, NUM_LAYERS);
            Some(TestGraph {
                graph,
                num_layers: NUM_LAYERS,
            })
        }
        2 => {
            let test_graph_index = args[1].parse::<usize>();
            if test_graph_index.is_err() {
                return None;
            }
            let test_graph_index = test_graph_index.unwrap();
            if test_graph_index >= TESTGRAPHS.len() {
                return None;
            }

            let graph = (*TESTGRAPHS[test_graph_index]).clone();
            Some(graph)
        }
        3 => {
            let num_nodes = args[1].parse::<usize>();
            let num_layers = args[2].parse::<usize>();
            if num_nodes.is_err() || num_layers.is_err() {
                return None;
            }

            let num_nodes = num_nodes.unwrap();
            let num_layers = num_layers.unwrap();
            let graph = generate_random_graph(num_nodes, num_layers);
            Some(TestGraph { graph, num_layers })
        }
        _ => None,
    }
}
