use cullcore::scorer::score_image;
use image::{GrayImage, Luma, DynamicImage};

#[test]
fn sharp_and_well_exposed_should_be_keep() {
    let mut img = GrayImage::new(100, 100);
    for (x, y, p) in img.enumerate_pixels_mut() {
        let v = if (x + y) % 2 == 0 { 120 } else { 135 };
        *p = Luma([v]);
    }

    let result = score_image(&DynamicImage::ImageLuma8(img));
    assert!(matches!(result.verdict, cullcore::verdict::Verdict::Keep));
}
