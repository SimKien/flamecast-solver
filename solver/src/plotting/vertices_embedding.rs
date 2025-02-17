use plotters::{
    prelude::{BitMapBackend, IntoDrawingArea},
    style::{RGBColor, WHITE},
};

use crate::VertexEmbedding;

use super::{create_node, ROOT_HEIGHT, ROOT_WIDTH};

#[derive(Debug, Clone)]
pub struct PlottingVertices {
    pub vertices: Vec<VertexEmbedding>,
    pub color: RGBColor,
}

impl PlottingVertices {
    pub fn new(vertices: Vec<VertexEmbedding>, color: RGBColor) -> Self {
        return PlottingVertices { vertices, color };
    }
}

pub fn plot_vertices_with_colors(
    file_path: &str,
    plotting_vertices: &Vec<PlottingVertices>,
    show_indices: bool,
) {
    let root = BitMapBackend::new(file_path, (ROOT_WIDTH, ROOT_HEIGHT)).into_drawing_area();

    // Set background color
    root.fill(&WHITE).unwrap();

    plotting_vertices
        .iter()
        .enumerate()
        .for_each(|(index, plotting_vertices)| {
            plotting_vertices
                .vertices
                .iter()
                .for_each(|vertex_embedding| {
                    let index = if show_indices { Some(index) } else { None };
                    root.draw(&create_node(
                        vertex_embedding,
                        plotting_vertices.color,
                        index,
                    ))
                    .unwrap();
                });
        });

    root.present().unwrap();
}
