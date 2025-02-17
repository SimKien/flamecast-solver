use plotters::{
    prelude::{BitMapBackend, Circle, EmptyElement, PathElement, Text},
    style::{IntoFont, RGBColor, ShapeStyle, BLACK},
};

use crate::VertexEmbedding;

use super::{convert_vertex_to_i32, NODE_RADIUS};

pub fn create_node(
    embedded_vertex: &VertexEmbedding,
    color: RGBColor,
    index: Option<usize>,
) -> plotters::element::ComposedElement<
    (i32, i32),
    BitMapBackend,
    Circle<(i32, i32), i32>,
    Text<'_, (i32, i32), String>,
> {
    let coordinates = convert_vertex_to_i32(embedded_vertex);

    let text = if let Some(index) = index {
        format!("{}", index)
    } else {
        "".to_string()
    };

    return EmptyElement::at(coordinates)
        + Circle::new((0, 0), NODE_RADIUS, ShapeStyle::from(color).filled())
        + Text::new(
            text,
            (2 * NODE_RADIUS, 0),
            ("sans-serif", 3 * NODE_RADIUS).into_font(),
        );
}

pub fn create_edge(source: &VertexEmbedding, target: &VertexEmbedding) -> PathElement<(i32, i32)> {
    let source_coordinates = convert_vertex_to_i32(source);
    let target_coordinates = convert_vertex_to_i32(target);

    return PathElement::new(vec![source_coordinates, target_coordinates], &BLACK);
}
