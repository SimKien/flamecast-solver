use std::sync::LazyLock;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{plotting::plot_flamecast_test_instance, VertexEmbeddings};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlamecastTestInstance {
    pub alpha: f64,
    pub num_layers: usize,
    pub capacities: Vec<usize>,
    pub sources_drains_embeddings: VertexEmbeddings,
}

impl FlamecastTestInstance {
    pub fn plot_instance(&self, file_path: &str) {
        plot_flamecast_test_instance(self, file_path);
    }
}

const FLAMECAST_TEST_INSTANCE1: LazyLock<FlamecastTestInstance> =
    LazyLock::new(|| FlamecastTestInstance {
        alpha: 0.0,
        num_layers: 3,
        capacities: vec![10, 10, 10],
        sources_drains_embeddings: VertexEmbeddings {
            embeddings: vec![vec![(0.25, 0.25), (0.25, 0.75)], vec![], vec![(0.5, 0.5)]],
        },
    });

const FLAMECAST_TEST_INSTANCE2: LazyLock<FlamecastTestInstance> =
    LazyLock::new(|| FlamecastTestInstance {
        alpha: 0.0,
        num_layers: 5,
        capacities: vec![1, 1, 1, 1, 2],
        sources_drains_embeddings: VertexEmbeddings {
            embeddings: vec![
                vec![(0.25, 0.25), (0.25, 0.75)],
                vec![],
                vec![],
                vec![],
                vec![(0.75, 0.5)],
            ],
        },
    });

const FLAMECAST_TEST_INSTANCE3: LazyLock<FlamecastTestInstance> =
    LazyLock::new(|| FlamecastTestInstance {
        alpha: 0.0,
        num_layers: 5,
        capacities: vec![1, 1, 2, 2, 3],
        sources_drains_embeddings: VertexEmbeddings {
            embeddings: vec![
                vec![
                    (0.25, 0.1),
                    (0.25, 0.2),
                    (0.25, 0.3),
                    (0.25, 0.4),
                    (0.25, 0.5),
                    (0.25, 0.6),
                    (0.25, 0.7),
                    (0.25, 0.8),
                    (0.25, 0.9),
                ],
                vec![],
                vec![],
                vec![],
                vec![(0.75, 0.2), (0.75, 0.4), (0.75, 0.6), (0.75, 0.8)],
            ],
        },
    });

pub const FLAMECAST_TEST_INSTANCES: [LazyLock<FlamecastTestInstance>; 3] = [
    FLAMECAST_TEST_INSTANCE1,
    FLAMECAST_TEST_INSTANCE2,
    FLAMECAST_TEST_INSTANCE3,
];

pub fn generate_random_flamecast_instance(
    num_layers: usize,
    num_sources: usize,
    num_drains: usize,
    clear_structure: bool,
) -> FlamecastTestInstance {
    let mut rng = rand::thread_rng();

    let alpha = rng.gen_range(0.0..=1.0);

    let mut max_capacity = if num_sources % num_drains == 0 {
        num_sources / num_drains
    } else {
        (num_sources / num_drains) + 1
    };
    max_capacity += rng.gen_range(0..=max_capacity);

    let mut capacities = (0..num_layers)
        .map(|_| rng.gen_range(1..=max_capacity))
        .collect::<Vec<usize>>();
    capacities.sort();
    capacities[num_layers - 1] = max_capacity;

    let mut sources_drains_embeddings = VertexEmbeddings::new_with_size(num_layers);
    (0..num_sources).for_each(|_| {
        let x = if clear_structure {
            0.2
        } else {
            rng.gen_range(0.0..=1.0)
        };
        let y = rng.gen_range(0.0..=1.0);
        sources_drains_embeddings.embeddings[0].push((x, y));
    });
    (0..num_drains).for_each(|_| {
        let x = if clear_structure {
            0.8
        } else {
            rng.gen_range(0.0..=1.0)
        };
        let y = rng.gen_range(0.0..=1.0);
        sources_drains_embeddings.embeddings[num_layers - 1].push((x, y));
    });

    FlamecastTestInstance {
        alpha,
        num_layers,
        capacities,
        sources_drains_embeddings,
    }
}
