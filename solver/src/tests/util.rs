use crate::VertexEmbeddings;

pub fn embeddings_equal(
    calculated_embeddings: &VertexEmbeddings,
    expected_embeddings: &VertexEmbeddings,
) -> bool {
    if calculated_embeddings.len() != expected_embeddings.len() {
        return false;
    }

    for (vertex, calculated_embedding) in calculated_embeddings.iter() {
        if !expected_embeddings.contains_key(vertex) {
            return false;
        }
        let expected_embedding = expected_embeddings.get(vertex).unwrap();
        if !float_equal(calculated_embedding.0, expected_embedding.0)
            || !float_equal(calculated_embedding.1, expected_embedding.1)
        {
            return false;
        }
    }
    return true;
}

pub fn float_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.0001
}
