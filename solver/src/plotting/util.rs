use crate::EmbeddedVertex;

use super::{ROOT_HEIGHT, ROOT_WIDTH};

pub fn convert_vertex_to_i32(vertex: &EmbeddedVertex) -> (i32, i32) {
    (
        (vertex.0 * ROOT_WIDTH as f32).round() as i32,
        (vertex.1 * ROOT_HEIGHT as f32).round() as i32,
    )
}
