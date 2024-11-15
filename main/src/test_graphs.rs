use std::sync::LazyLock;

use solver::{DirectedGraph, VertexEmbeddings};

#[derive(Debug, Clone)]
pub struct TestGraph {
    pub graph: DirectedGraph,
    pub source_embeddings: VertexEmbeddings,
    pub drain_embeddings: VertexEmbeddings,
    pub alpha: f64,
}

const GRAPH_1: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3],
        edges: vec![(0, 2), (1, 2), (2, 3)],
    },
    source_embeddings: {
        let mut source_embeddings = VertexEmbeddings::new();
        source_embeddings.insert(0, (0.25, 0.25));
        source_embeddings.insert(1, (0.25, 0.75));
        source_embeddings
    },
    drain_embeddings: {
        let mut drain_embeddings = VertexEmbeddings::new();
        drain_embeddings.insert(3, (0.75, 0.5));
        drain_embeddings
    },
    alpha: 0.0,
});

const GRAPH_2: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3, 4],
        edges: vec![(0, 3), (1, 3), (2, 3), (3, 4)],
    },
    source_embeddings: {
        let mut source_embeddings = VertexEmbeddings::new();
        source_embeddings.insert(0, (0.25, 0.25));
        source_embeddings.insert(1, (0.25, 0.5));
        source_embeddings.insert(2, (0.25, 0.75));
        source_embeddings
    },
    drain_embeddings: {
        let mut drain_embeddings = VertexEmbeddings::new();
        drain_embeddings.insert(4, (0.75, 0.5));
        drain_embeddings
    },
    alpha: 0.0,
});

const GRAPH_3: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3, 4, 5],
        edges: vec![(0, 4), (1, 4), (2, 4), (3, 4), (4, 5)],
    },
    source_embeddings: {
        let mut source_embeddings = VertexEmbeddings::new();
        source_embeddings.insert(0, (0.25, 0.25));
        source_embeddings.insert(1, (0.25, 0.375));
        source_embeddings.insert(2, (0.25, 0.625));
        source_embeddings.insert(3, (0.25, 0.75));
        source_embeddings
    },
    drain_embeddings: {
        let mut drain_embeddings = VertexEmbeddings::new();
        drain_embeddings.insert(5, (0.75, 0.5));
        drain_embeddings
    },
    alpha: 0.0,
});

const GRAPH_4: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3, 4],
        edges: vec![(0, 2), (1, 2), (2, 3), (3, 4)],
    },
    source_embeddings: {
        let mut source_embeddings = VertexEmbeddings::new();
        source_embeddings.insert(0, (0.25, 0.25));
        source_embeddings.insert(1, (0.25, 0.75));
        source_embeddings
    },
    drain_embeddings: {
        let mut drain_embeddings = VertexEmbeddings::new();
        drain_embeddings.insert(4, (0.75, 0.5));
        drain_embeddings
    },
    alpha: 0.0,
});

const GRAPH_5: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3, 4, 5],
        edges: vec![(0, 4), (1, 4), (2, 4), (3, 4), (4, 5)],
    },
    source_embeddings: {
        let mut source_embeddings = VertexEmbeddings::new();
        source_embeddings.insert(0, (0.5, 0.75));
        source_embeddings.insert(1, (0.75, 0.5));
        source_embeddings.insert(2, (0.5, 0.25));
        source_embeddings.insert(3, (0.25, 0.5));
        source_embeddings
    },
    drain_embeddings: {
        let mut drain_embeddings = VertexEmbeddings::new();
        drain_embeddings.insert(5, (0.5, 0.5));
        drain_embeddings
    },
    alpha: 0.0,
});

const GRAPH_6: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3, 4, 5, 6],
        edges: vec![(0, 5), (1, 5), (2, 5), (3, 5), (4, 5), (5, 6)],
    },
    source_embeddings: {
        let mut source_embeddings = VertexEmbeddings::new();
        source_embeddings.insert(0, (0.75, 0.5));
        source_embeddings.insert(1, (0.5773, 0.7378));
        source_embeddings.insert(2, (0.2977, 0.6469));
        source_embeddings.insert(3, (0.2977, 0.3531));
        source_embeddings.insert(4, (0.5773, 0.2622));
        source_embeddings
    },
    drain_embeddings: {
        let mut drain_embeddings = VertexEmbeddings::new();
        drain_embeddings.insert(6, (0.5, 0.5));
        drain_embeddings
    },
    alpha: 0.0,
});

const GRAPH_7: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3, 4, 5, 6, 7],
        edges: vec![(0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (6, 7)],
    },
    source_embeddings: {
        let mut source_embeddings = VertexEmbeddings::new();
        source_embeddings.insert(0, (0.75, 0.5));
        source_embeddings.insert(1, (0.625, 0.7165));
        source_embeddings.insert(2, (0.375, 0.7165));
        source_embeddings.insert(3, (0.25, 0.5));
        source_embeddings.insert(4, (0.375, 0.2835));
        source_embeddings.insert(5, (0.625, 0.2835));
        source_embeddings
    },
    drain_embeddings: {
        let mut drain_embeddings = VertexEmbeddings::new();
        drain_embeddings.insert(7, (0.5, 0.5));
        drain_embeddings
    },
    alpha: 0.0,
});

const GRAPH_8: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        edges: vec![
            (0, 3),
            (1, 3),
            (2, 3),
            (3, 4),
            (4, 5),
            (6, 7),
            (7, 4),
            (8, 9),
            (9, 4),
        ],
    },
    source_embeddings: {
        let mut source_embeddings = VertexEmbeddings::new();
        source_embeddings.insert(0, (0.25, 0.65));
        source_embeddings.insert(1, (0.1, 0.5));
        source_embeddings.insert(2, (0.25, 0.35));
        source_embeddings.insert(6, (0.5, 0.65));
        source_embeddings.insert(8, (0.5, 0.35));
        source_embeddings
    },
    drain_embeddings: {
        let mut drain_embeddings = VertexEmbeddings::new();
        drain_embeddings.insert(5, (0.75, 0.5));
        drain_embeddings
    },
    alpha: 0.0,
});

pub const TESTGRAPHS: [LazyLock<TestGraph>; 8] = [
    GRAPH_1, GRAPH_2, GRAPH_3, GRAPH_4, GRAPH_5, GRAPH_6, GRAPH_7, GRAPH_8,
];

pub fn combine_test_graphs(graph1: &mut TestGraph, graph2: &TestGraph) -> Option<TestGraph> {
    // Check if the graphs have the same number of layers
    if graph1.graph.get_layers().len() != graph2.graph.get_layers().len() {
        return None;
    }

    let mut new_graph = graph1.clone();
    let max_vertex = graph1.graph.vertices.iter().max().unwrap();

    new_graph.graph.vertices.extend(
        graph2
            .graph
            .vertices
            .iter()
            .cloned()
            .map(|v| v + max_vertex + 1),
    );
    new_graph.graph.edges.extend(
        graph2
            .graph
            .edges
            .iter()
            .map(|(u, v)| (*u + max_vertex + 1, *v + max_vertex + 1)),
    );
    new_graph.source_embeddings.extend(
        graph2
            .source_embeddings
            .iter()
            .map(|(v, e)| (*v + max_vertex + 1, *e)),
    );
    new_graph.drain_embeddings.extend(
        graph2
            .drain_embeddings
            .iter()
            .map(|(v, e)| (*v + max_vertex + 1, *e)),
    );

    return Some(new_graph);
}
