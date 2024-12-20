use std::sync::LazyLock;

use crate::{Layer, LayeredGraph, Vertex, VertexEmbeddings};

#[derive(Debug, Clone)]
pub struct TestGraph {
    pub graph: LayeredGraph,
    pub sources_drains_embeddings: VertexEmbeddings,
    pub alpha: f64,
}

const GRAPH_1: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![Vertex::new(Some(0), None), Vertex::new(Some(0), None)]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0, 1]))]),
        Layer::from(vec![Vertex::new(None, Some(vec![0]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![(0.25, 0.25), (0.25, 0.75)],
        vec![],
        vec![(0.75, 0.5)],
    ]),
    alpha: 0.0,
});

const GRAPH_2: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
        ]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0, 1, 2]))]),
        Layer::from(vec![Vertex::new(None, Some(vec![0]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![(0.25, 0.25), (0.25, 0.5), (0.25, 0.75)],
        vec![],
        vec![(0.75, 0.5)],
    ]),
    alpha: 0.0,
});

const GRAPH_3: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
        ]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0, 1, 2, 3]))]),
        Layer::from(vec![Vertex::new(None, Some(vec![0]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![(0.25, 0.25), (0.25, 0.375), (0.25, 0.625), (0.25, 0.75)],
        vec![],
        vec![(0.75, 0.5)],
    ]),
    alpha: 0.0,
});

const GRAPH_4: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![Vertex::new(Some(0), None), Vertex::new(Some(0), None)]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0, 1]))]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0]))]),
        Layer::from(vec![Vertex::new(None, Some(vec![0]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![(0.25, 0.25), (0.25, 0.75)],
        vec![],
        vec![],
        vec![(0.75, 0.5)],
    ]),
    alpha: 0.0,
});

const GRAPH_5: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
        ]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0, 1, 2, 3]))]),
        Layer::from(vec![Vertex::new(None, Some(vec![0]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![(0.5, 0.75), (0.75, 0.5), (0.5, 0.25), (0.25, 0.5)],
        vec![],
        vec![(0.5, 0.5)],
    ]),
    alpha: 0.0,
});

const GRAPH_6: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
        ]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0, 1, 2, 3, 4]))]),
        Layer::from(vec![Vertex::new(None, Some(vec![0]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![
            (0.75, 0.5),
            (0.5773, 0.7378),
            (0.2977, 0.6469),
            (0.2977, 0.3531),
            (0.5773, 0.2622),
        ],
        vec![],
        vec![(0.5, 0.5)],
    ]),
    alpha: 0.0,
});

const GRAPH_7: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
        ]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0, 1, 2, 3, 4, 5]))]),
        Layer::from(vec![Vertex::new(None, Some(vec![0]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![
            (0.75, 0.5),
            (0.625, 0.7165),
            (0.375, 0.7165),
            (0.25, 0.5),
            (0.375, 0.2835),
            (0.625, 0.2835),
        ],
        vec![],
        vec![(0.5, 0.5)],
    ]),
    alpha: 0.0,
});

const GRAPH_8: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(1), None),
            Vertex::new(Some(2), None),
        ]),
        Layer::from(vec![
            Vertex::new(Some(0), Some(vec![0, 1, 2])),
            Vertex::new(Some(0), Some(vec![3])),
            Vertex::new(Some(0), Some(vec![4])),
        ]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0]))]),
        Layer::from(vec![Vertex::new(None, Some(vec![0]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![
            (0.25, 0.65),
            (0.1, 0.5),
            (0.25, 0.35),
            (0.5, 0.65),
            (0.5, 0.35),
        ],
        vec![],
        vec![],
        vec![(0.75, 0.5)],
    ]),
    alpha: 0.0,
});

const GRAPH_9: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
        ]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0, 1, 2]))]),
        Layer::from(vec![Vertex::new(None, Some(vec![0]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![(0.25, 0.25), (0.25, 0.5), (0.25, 0.75)],
        vec![],
        vec![(0.75, 0.5)],
    ]),
    alpha: 0.8,
});

const GRAPH_10: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(1), None),
            Vertex::new(Some(2), None),
        ]),
        Layer::from(vec![
            Vertex::new(Some(0), Some(vec![0, 1, 2])),
            Vertex::new(Some(0), Some(vec![3])),
            Vertex::new(Some(0), Some(vec![4])),
        ]),
        Layer::from(vec![Vertex::new(Some(0), Some(vec![0, 1, 2]))]),
        Layer::from(vec![Vertex::new(None, Some(vec![0]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![
            (0.25, 0.65),
            (0.1, 0.5),
            (0.25, 0.35),
            (0.5, 0.65),
            (0.5, 0.35),
        ],
        vec![],
        vec![],
        vec![(0.75, 0.5)],
    ]),
    alpha: 0.9,
});

const GRAPH_11: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: LayeredGraph::from(vec![
        Layer::from(vec![
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(0), None),
            Vertex::new(Some(1), None),
            Vertex::new(Some(1), None),
            Vertex::new(Some(1), None),
            Vertex::new(Some(1), None),
            Vertex::new(Some(1), None),
            Vertex::new(Some(1), None),
            Vertex::new(Some(1), None),
            Vertex::new(Some(2), None),
            Vertex::new(Some(2), None),
            Vertex::new(Some(2), None),
            Vertex::new(Some(2), None),
            Vertex::new(Some(2), None),
        ]),
        Layer::from(vec![
            Vertex::new(Some(0), Some(vec![0, 1, 2, 3])),
            Vertex::new(Some(0), Some(vec![4, 5, 6, 7, 8, 9, 10])),
            Vertex::new(Some(1), Some(vec![11, 12, 13, 14, 15])),
        ]),
        Layer::from(vec![
            Vertex::new(Some(0), Some(vec![0, 1])),
            Vertex::new(Some(0), Some(vec![2])),
        ]),
        Layer::from(vec![Vertex::new(None, Some(vec![0, 1]))]),
    ]),
    sources_drains_embeddings: VertexEmbeddings::from(vec![
        vec![
            (0.1, 0.3),
            (0.22, 0.3),
            (0.29, 0.35),
            (0.2, 0.1),
            (0.4, 0.88),
            (0.5, 0.78),
            (0.55, 0.95),
            (0.6, 0.85),
            (0.52, 0.75),
            (0.52, 0.84),
            (0.4243, 0.798),
            (0.82, 0.2),
            (0.78, 0.37),
            (0.86, 0.4),
            (0.84, 0.42),
            (0.92, 0.35),
        ],
        vec![],
        vec![],
        vec![(0.6, 0.5)],
    ]),
    alpha: 0.0,
});

pub const TESTGRAPHS: [LazyLock<TestGraph>; 11] = [
    GRAPH_1, GRAPH_2, GRAPH_3, GRAPH_4, GRAPH_5, GRAPH_6, GRAPH_7, GRAPH_8, GRAPH_9, GRAPH_10,
    GRAPH_11,
];

// combine two graphs into one, alpha of graph1 is used and alpha of graph2 is ignored
pub fn combine_test_graphs(graph1: &mut TestGraph, graph2: &TestGraph) -> Option<TestGraph> {
    // Check if the graphs have the same number of layers
    if graph1.graph.layers.len() != graph2.graph.layers.len() {
        return None;
    }

    let mut new_graph = graph1.clone();

    graph2
        .graph
        .layers
        .iter()
        .enumerate()
        .for_each(|(layer_index, layer)| {
            new_graph.graph.layers[layer_index]
                .vertices
                .extend(layer.vertices.iter().map(|vertex| {
                    let mut vertex = vertex.clone();
                    if let Some(parent_index) = vertex.parent_index {
                        let parent_layer_base_index =
                            graph1.graph.layers[parent_index + 1].vertices.len();
                        vertex.parent_index = Some(parent_index + parent_layer_base_index);
                    }
                    if let Some(children_indices) = &vertex.children_indices {
                        let child_layer_base_index =
                            graph1.graph.layers[layer_index - 1].vertices.len();
                        let new_children = children_indices
                            .iter()
                            .map(|child| child + child_layer_base_index)
                            .collect();
                        vertex.children_indices = Some(new_children);
                    }
                    vertex
                }));
        });

    new_graph.sources_drains_embeddings.embeddings[0]
        .extend(graph2.sources_drains_embeddings.embeddings[0].iter());
    new_graph.sources_drains_embeddings.embeddings[new_graph.graph.layers.len() - 1].extend(
        graph2.sources_drains_embeddings.embeddings[new_graph.graph.layers.len() - 1].iter(),
    );

    return Some(new_graph);
}

#[cfg(test)]
pub type OptimalEmbeddings = VertexEmbeddings;

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct EmbeddingSample {
    pub valid: bool,
    pub embeddings: OptimalEmbeddings,
}

#[cfg(test)]
const EMPTY_EMBEDDING: LazyLock<EmbeddingSample> = LazyLock::new(|| EmbeddingSample {
    valid: false,
    embeddings: VertexEmbeddings::new(),
});

#[cfg(test)]
const OPTIMAL_EMBEDDING_5: LazyLock<EmbeddingSample> = LazyLock::new(|| EmbeddingSample {
    valid: true,
    embeddings: VertexEmbeddings::from(vec![
        vec![(0.5, 0.75), (0.75, 0.5), (0.5, 0.25), (0.25, 0.5)],
        vec![(0.5, 0.5)],
        vec![(0.5, 0.5)],
    ]),
});

#[cfg(test)]
const OPTIMAL_EMBEDDING_6: LazyLock<EmbeddingSample> = LazyLock::new(|| EmbeddingSample {
    valid: true,
    embeddings: VertexEmbeddings::from(vec![
        vec![
            (0.75, 0.5),
            (0.5773, 0.7378),
            (0.2977, 0.6469),
            (0.2977, 0.3531),
            (0.5773, 0.2622),
        ],
        vec![(0.49999999998516265, 0.5)],
        vec![(0.5, 0.5)],
    ]),
});

#[cfg(test)]
const OPTIMAL_EMBEDDING_7: LazyLock<EmbeddingSample> = LazyLock::new(|| EmbeddingSample {
    valid: true,
    embeddings: VertexEmbeddings::from(vec![
        vec![
            (0.75, 0.5),
            (0.625, 0.7165),
            (0.375, 0.7165),
            (0.25, 0.5),
            (0.375, 0.2835),
            (0.625, 0.2835),
        ],
        vec![(0.5, 0.5)],
        vec![(0.5, 0.5)],
    ]),
});

#[cfg(test)]
pub const TESTGRAPHS_EMBEDDING_SAMPLES: [LazyLock<EmbeddingSample>; 11] = [
    EMPTY_EMBEDDING,
    EMPTY_EMBEDDING,
    EMPTY_EMBEDDING,
    EMPTY_EMBEDDING,
    OPTIMAL_EMBEDDING_5,
    OPTIMAL_EMBEDDING_6,
    OPTIMAL_EMBEDDING_7,
    EMPTY_EMBEDDING,
    EMPTY_EMBEDDING,
    EMPTY_EMBEDDING,
    EMPTY_EMBEDDING,
];
