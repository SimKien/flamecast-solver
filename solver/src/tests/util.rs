use crate::{Vertex, VertexEmbeddings};

use super::EPSILON;

pub fn embeddings_equal(
    calculated_embeddings: &VertexEmbeddings,
    expected_embeddings: &VertexEmbeddings,
) -> bool {
    if calculated_embeddings.embeddings.len() != expected_embeddings.embeddings.len() {
        return false;
    }

    for (calculated_layer, expected_layer) in calculated_embeddings
        .embeddings
        .iter()
        .zip(expected_embeddings.embeddings.iter())
    {
        if calculated_layer.len() != expected_layer.len() {
            return false;
        }
        for (calculated_embedding, expected_embedding) in
            calculated_layer.iter().zip(expected_layer.iter())
        {
            if !float_equal(calculated_embedding.0, expected_embedding.0)
                || !float_equal(calculated_embedding.1, expected_embedding.1)
            {
                return false;
            }
        }
    }

    return true;
}

pub fn float_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn vertices_equal(vertex1: &Vertex, vertex2: &Vertex) -> bool {
    if (vertex1.children_indices.is_none() && vertex2.children_indices.is_some())
        || (vertex1.children_indices.is_some() && vertex2.children_indices.is_none())
        || (vertex1.parent_index.is_none() && vertex2.parent_index.is_some())
        || (vertex1.parent_index.is_some() && vertex2.parent_index.is_none())
    {
        return false;
    }

    if let Some(children1) = vertex1.children_indices.as_ref() {
        let children2 = vertex2.children_indices.as_ref().unwrap();

        if children1.len() != children2.len() {
            return false;
        }

        for child1 in children1.iter() {
            if !children2.contains(child1) {
                return false;
            }
        }
    }

    if let Some(parent1) = vertex1.parent_index.as_ref() {
        let parent2 = vertex2.parent_index.as_ref().unwrap();
        if parent1 != parent2 {
            return false;
        }
    }

    return true;
}
