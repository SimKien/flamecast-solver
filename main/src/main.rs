use rand::Rng;
use solver::{
    embed_graph, generate_random_graph, get_layers, hello_world, plot_graph, VertexEmbeddings,
};

pub const NUM_NODES: usize = 200;
pub const NUM_LAYERS: usize = 5;
pub const ALPHA: f32 = 0.0;

fn main() {
    hello_world();

    let random_graph = generate_random_graph(NUM_NODES, NUM_LAYERS);

    let x_delta = 1.0 / ((NUM_LAYERS) as f32);
    let side_distance = x_delta / 2.0;

    let mut rng = rand::thread_rng();
    let mut sources_embeddings = VertexEmbeddings::new();
    let mut drains_embeddings = VertexEmbeddings::new();

    let layers = get_layers(&random_graph);
    for source in layers[0].iter() {
        let y = rng.gen_range(0.0..=1.0);
        sources_embeddings.insert(*source, (side_distance, y));
    }
    for drain in layers[NUM_LAYERS - 1].iter() {
        let y = rng.gen_range(0.0..=1.0);
        drains_embeddings.insert(*drain, (1.0 - side_distance, y));
    }

    let embedded_graph = embed_graph(random_graph, ALPHA, &sources_embeddings, &drains_embeddings);

    plot_graph(&embedded_graph, true, true);
}
