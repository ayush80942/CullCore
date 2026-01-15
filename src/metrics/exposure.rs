use image::DynamicImage;

/// Returns exposure score (0.0 = bad exposure, 1.0 = well exposed)
pub fn exposure_score(img: &DynamicImage) -> f32 {
    let gray = img.to_luma8();

    let mut dark = 0u32;
    let mut bright = 0u32;
    let total = gray.width() * gray.height();

    for p in gray.pixels() {
        let v = p[0];
        if v < 20 {
            dark += 1;
        } else if v > 235 {
            bright += 1;
        }
    }

    let dark_ratio = dark as f32 / total as f32;
    let bright_ratio = bright as f32 / total as f32;

    let score = 1.0 - (dark_ratio + bright_ratio);
    score.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};

    #[test]
    fn well_exposed_image_scores_higher() {
        let mut good = GrayImage::new(100, 100);
        for p in good.pixels_mut() {
            *p = Luma([128]);
        }

        let mut dark = GrayImage::new(100, 100);
        for p in dark.pixels_mut() {
            *p = Luma([5]);
        }

        let good_score = exposure_score(&DynamicImage::ImageLuma8(good));
        let dark_score = exposure_score(&DynamicImage::ImageLuma8(dark));

        assert!(good_score > dark_score);
    }
}