use plotters::{
    prelude::{BitMapBackend, IntoDrawingArea},
    style::WHITE,
};

use crate::tests::FlamecastTestInstance;

use super::{create_node, DISTINCT_COLORS, ROOT_HEIGHT, ROOT_WIDTH};

pub fn plot_flamecast_test_instance(
    instance: &FlamecastTestInstance,
    file_path: &str,
    show_capacities: bool,
) {
    let root = BitMapBackend::new(file_path, (ROOT_WIDTH, ROOT_HEIGHT)).into_drawing_area();

    // Set background color
    root.fill(&WHITE).unwrap();

    let sources_color = DISTINCT_COLORS[0];
    let drains_color = DISTINCT_COLORS[1];

    let sources_capacity = instance.capacities[0];
    let drains_capacity = instance.capacities[instance.capacities.len() - 1];

    instance.sources_drains_embeddings.embeddings[0]
        .iter()
        .for_each(|embedding| {
            let index = if show_capacities {
                Some(sources_capacity)
            } else {
                None
            };
            root.draw(&create_node(embedding, sources_color, index))
                .unwrap();
        });

    instance.sources_drains_embeddings.embeddings
        [instance.sources_drains_embeddings.embeddings.len() - 1]
        .iter()
        .for_each(|embedding| {
            let index = if show_capacities {
                Some(drains_capacity)
            } else {
                None
            };
            root.draw(&create_node(embedding, drains_color, index))
                .unwrap();
        });

    root.present().unwrap();
}
