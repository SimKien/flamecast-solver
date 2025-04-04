#[cfg(test)]
use rand::Rng;

#[cfg(test)]
use crate::VertexEmbedding;

#[cfg(test)]
pub fn create_random_source_embeddings(sources_size: usize) -> Vec<VertexEmbedding> {
    let mut rng = rand::thread_rng();
    let mut sources_embeddings = Vec::new();

    for _ in 0..sources_size {
        let x = rng.gen_range(0.0..=1.0);
        let y = rng.gen_range(0.0..=1.0);

        sources_embeddings.push((x, y));
    }

    return sources_embeddings;
}
