use std::sync::LazyLock;

use solver::DirectedGraph;

#[derive(Debug, Clone)]
pub struct TestGraph {
    pub graph: DirectedGraph,
    pub num_layers: usize,
}

const GRAPH_1: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3],
        edges: vec![(0, 2), (1, 2), (2, 3)],
    },
    num_layers: 3,
});

const GRAPH_2: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3, 4],
        edges: vec![(0, 3), (1, 3), (2, 3), (3, 4)],
    },
    num_layers: 3,
});

const GRAPH_3: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3, 4, 5],
        edges: vec![(0, 4), (1, 4), (2, 4), (3, 4), (4, 5)],
    },
    num_layers: 3,
});

const GRAPH_4: LazyLock<TestGraph> = LazyLock::new(|| TestGraph {
    graph: DirectedGraph {
        vertices: vec![0, 1, 2, 3, 4],
        edges: vec![(0, 2), (1, 2), (2, 3), (3, 4)],
    },
    num_layers: 4,
});

pub const TESTGRAPHS: [LazyLock<TestGraph>; 4] = [GRAPH_1, GRAPH_2, GRAPH_3, GRAPH_4];
