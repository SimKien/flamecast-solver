use crate::VertexEmbeddings;

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
