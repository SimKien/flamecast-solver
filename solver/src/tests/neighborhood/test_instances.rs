use std::sync::LazyLock;

use crate::{FlamecastInstance, GraphEmbedding, Layer, LayeredGraph, Vertex, VertexEmbeddings};

const NEIGHBORHOOD_TEST_INSTANCE1: LazyLock<FlamecastInstance> =
    LazyLock::new(|| FlamecastInstance {
        alpha: 0.8,
        num_layers: 5,
        capacities: vec![1, 2, 3, 4, 4],
        sources_drains_embeddings: VertexEmbeddings {
            embeddings: vec![
                vec![(0.25, 0.2), (0.25, 0.4), (0.25, 0.6), (0.25, 0.8)],
                vec![],
                vec![],
                vec![],
                vec![(0.75, 0.5)],
            ],
        },
        current_solution: GraphEmbedding::new(
            LayeredGraph::from(vec![
                Layer::from(vec![
                    Vertex::new(Some(0), None),
                    Vertex::new(Some(1), None),
                    Vertex::new(Some(2), None),
                    Vertex::new(Some(3), None),
                ]),
                Layer::from(vec![
                    Vertex::new(Some(0), Some(vec![0])),
                    Vertex::new(Some(0), Some(vec![1])),
                    Vertex::new(Some(1), Some(vec![2])),
                    Vertex::new(Some(2), Some(vec![3])),
                ]),
                Layer::from(vec![
                    Vertex::new(Some(0), Some(vec![0, 1])),
                    Vertex::new(Some(0), Some(vec![2])),
                    Vertex::new(Some(1), Some(vec![3])),
                ]),
                Layer::from(vec![
                    Vertex::new(Some(0), Some(vec![0, 1])),
                    Vertex::new(Some(0), Some(vec![2])),
                ]),
                Layer::from(vec![Vertex::new(None, Some(vec![0, 1]))]),
            ]),
            VertexEmbeddings::new(),
        ),
    });

pub const NEIGHBORHOOD_TEST_INSTANCES: [LazyLock<FlamecastInstance>; 1] =
    [NEIGHBORHOOD_TEST_INSTANCE1];
