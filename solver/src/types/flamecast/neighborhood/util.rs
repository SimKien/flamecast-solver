use clustering::kmeans;

use crate::{VertexEmbeddings, VertexID};

pub fn cluster_children(
    children: &Vec<VertexID>,
    embeddings: &VertexEmbeddings,
) -> (Vec<VertexID>, Vec<VertexID>) {
    // Use kmeans to cluster children into two clusters
    let sample_dimension = 2;
    let max_iter = 40;
    let mut samples = vec![vec![0.0; sample_dimension]; children.len()];

    children.iter().enumerate().for_each(|(i, child)| {
        samples[i] = vec![
            embeddings.embeddings[child.layer][child.index].0,
            embeddings.embeddings[child.layer][child.index].1,
        ];
    });

    let result = kmeans(2, &samples, max_iter);

    let mut cluster1 = Vec::new();
    let mut cluster2 = Vec::new();
    for (i, child) in children.iter().enumerate() {
        if result.membership[i] == 0 {
            cluster1.push(child.clone());
        } else {
            cluster2.push(child.clone());
        }
    }

    if cluster1.len() == 0 {
        cluster1.push(cluster2.pop().unwrap());
    } else if cluster2.len() == 0 {
        cluster2.push(cluster1.pop().unwrap());
    }

    return (cluster1, cluster2);
}
