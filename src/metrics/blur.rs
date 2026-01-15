use image::DynamicImage;
use imageproc::filter::laplacian_filter;

/// Returns a normalized blur score (0.0 = very blurry, 1.0 = very sharp)
pub fn blur_score(img: &DynamicImage) -> f32 {
    let gray  = img.to_luma8();

    let lap = laplacian_filter(&gray);

    let mut sum = 0.0f64;
    let mut sum_sq = 0.0f64;
    let mut count = 0.0f64;

    for pixel in lap.pixels() {
        let v = pixel[0] as f64;
        sum += v;
        sum_sq += v * v;
        count += 1.0;
    }

    let mean = sum / count;
    let variance = (sum_sq / count) - (mean * mean);

    normalize_variance(variance)
}

/// Maps raw variance into 0.0–1.0 range
fn normalize_variance(var: f64) -> f32 {
    let clamped = var.min(1000.0); // empirical upper bound
    (clamped / 1000.0) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};

    #[test]
    fn sharp_image_has_higher_score() {
        let mut sharp = GrayImage::new(100, 100);
        for (x, y, p) in sharp.enumerate_pixels_mut() {
            let v = if (x + y) % 2 == 0 { 0 } else { 255 };
            *p = Luma([v]);
        }

        let blurred = image::imageops::blur(&sharp, 5.0);

        let sharp_score = blur_score(&DynamicImage::ImageLuma8(sharp));
        let blurred_score = blur_score(&DynamicImage::ImageLuma8(blurred));

        assert!(sharp_score > blurred_score);
    }
}

