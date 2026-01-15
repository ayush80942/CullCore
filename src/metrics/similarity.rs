use image::DynamicImage;
use img_hash::{HasherConfig, ImageHash};

pub fn compute_phash(img: &DynamicImage) -> ImageHash {
    let hasher = HasherConfig::new()
        .hash_size(8, 8)
        .to_hasher();

    // Convert to grayscale in OUR crate
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    let raw = gray.into_raw(); // Vec<u8>

    // ✅ Rebuild ImageBuffer INSIDE img_hash crate
    let img_hash_buf =
        img_hash::image::ImageBuffer::from_raw(w, h, raw)
            .expect("valid buffer");

    let img_hash_img =
        img_hash::image::DynamicImage::ImageLuma8(img_hash_buf);

    hasher.hash_image(&img_hash_img)
}

/// Returns similarity score between 0.0 (very different) and 1.0 (identical)
pub fn hash_similarity(a: &ImageHash, b: &ImageHash) -> f32 {
    let dist = a.dist(b) as f32;
    let max_dist = (a.as_bytes().len() * 8) as f32;

    1.0 - (dist / max_dist)
}

// pub fn similarity_against_group(
//     current: &ImageHash,
//     others: &[ImageHash],
// ) -> f32 {
//     if others.is_empty() {
//         return 1.0;
//     }

//     let mut best = 0.0;

//     for h in others {
//         let s = hash_similarity(current, h);
//         if s > best {
//             best = s;
//         }
//     }

//     best
// }
