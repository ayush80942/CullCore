use img_hash::ImageHash;
use crate::metrics::similarity::hash_similarity;

/// Cluster images based on perceptual hash similarity
///
/// Returns Vec of clusters, each cluster is a Vec of image indices
pub fn cluster_images(
    hashes: &[ImageHash],
    threshold: f32,
) -> Vec<Vec<usize>> {
    let mut clusters: Vec<Vec<usize>> = Vec::new();

    'outer: for (i, h) in hashes.iter().enumerate() {
        for cluster in clusters.iter_mut() {
            // Use first image in cluster as representative
            let rep_index = cluster[0];
            let rep_hash = &hashes[rep_index];

            if hash_similarity(h, rep_hash) >= threshold {
                cluster.push(i);
                continue 'outer;
            }
        }

        // No matching cluster found → create new
        clusters.push(vec![i]);
    }

    clusters
}
