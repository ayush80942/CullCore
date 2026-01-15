use crate::metrics::{
    blur::blur_score,
    exposure::exposure_score,
    similarity::{compute_phash, hash_similarity},
};
use crate::verdict::Verdict;
use image::DynamicImage;
use img_hash::ImageHash;
use rayon::prelude::*;

pub struct ImageScore {
    pub blur: f32,
    pub exposure: f32,
    pub similarity: f32,
    pub final_score: f32,
    pub verdict: Verdict,
}

pub fn score_images(images: &[DynamicImage]) -> Vec<ImageScore> {
    let hashes: Vec<ImageHash> = images
        .iter()
        .map(compute_phash)
        .collect();

    images
        .par_iter()
        .enumerate()
        .map(|(i, img)| {
            let blur = blur_score(img);
            let exposure = exposure_score(img);

            let current = &hashes[i];

            let similarity = hashes
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, h)| hash_similarity(current, h))
                .fold(0.0, f32::max);

            let final_score = (
                0.45 * blur +
                0.35 * exposure +
                0.20 * similarity
            ).clamp(0.0, 1.0);

            let verdict = Verdict::from_score(final_score);

            ImageScore {
                blur,
                exposure,
                similarity,
                final_score,
                verdict,
            }
        })
        .collect()
}
