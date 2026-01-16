use crate::clustering::cluster_images;
use crate::metrics::{blur::blur_score, exposure::exposure_score, similarity::compute_phash};
use crate::verdict::Verdict;
use image::DynamicImage;
use img_hash::ImageHash;
use rayon::prelude::*;

pub struct ImageScore {
    pub blur: f32,
    pub exposure: f32,
    pub cluster_id: usize,
    pub final_score: f32,
    pub verdict: Verdict,
}

pub struct ScoringResult {
    pub scores: Vec<ImageScore>,
    pub clusters: Vec<Vec<usize>>,
}

pub fn score_images_with_clustering(
    images: &[DynamicImage],
    cluster_threshold: f32,
) -> ScoringResult {
    // ───────────── Step 1: Hashing ─────────────
    let hashes: Vec<ImageHash> = images.iter().map(compute_phash).collect();

    // ───────────── Step 2: Clustering ─────────────
    let clusters = cluster_images(&hashes, cluster_threshold);

    // image index → cluster id
    let mut cluster_map = vec![0usize; images.len()];
    for (cid, cluster) in clusters.iter().enumerate() {
        for &idx in cluster {
            cluster_map[idx] = cid;
        }
    }

    // ───────────── Step 3: Quality Scoring (Parallel) ─────────────
    let quality_scores: Vec<(f32, f32)> = images
        .par_iter()
        .map(|img| {
            let blur = blur_score(img);
            let exposure = exposure_score(img);
            (blur, exposure)
        })
        .collect();

    // ───────────── Step 4: Best Image per Cluster ─────────────
    let mut best_in_cluster = vec![0usize; clusters.len()];
    for (cid, cluster) in clusters.iter().enumerate() {
        best_in_cluster[cid] = *cluster
            .iter()
            .max_by(|&&a, &&b| {
                let qa = quality_scores[a].0 + quality_scores[a].1;
                let qb = quality_scores[b].0 + quality_scores[b].1;
                qa.partial_cmp(&qb).unwrap()
            })
            .unwrap();
    }

    // ───────────── Step 5: FINAL SCORE + VERDICT (YOUR CODE GOES HERE) ─────────────
    let scores = images
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let (blur, exposure) = quality_scores[i];
            let cid = cluster_map[i];

            // ⭐ cluster-based boost
            let quality = 0.55 * blur + 0.45 * exposure;

            let cluster_factor = if best_in_cluster[cid] == i { 1.0 } else { 0.25 };

            let final_score = (0.70 * quality + 0.30 * cluster_factor).clamp(0.0, 1.0);

            let verdict = Verdict::from_score(final_score);

            ImageScore {
                blur,
                exposure,
                cluster_id: cid,
                final_score,
                verdict,
            }
        })
        .collect();

    ScoringResult { scores, clusters }
}
