mod embedding_test_graphs;
mod flamecast_test_instances;

pub use embedding_test_graphs::*;
pub use flamecast_test_instances::*;

#[cfg(test)]
pub const EPSILON: f64 = 0.000001;
#[cfg(test)]
pub const WEISZFELD_EPSILON: f64 = 0.0000001;

#[cfg(test)]
mod graph_embedding;

#[cfg(test)]
mod util;

#[cfg(test)]
pub use util::*;
