use criterion::{criterion_group, criterion_main, Criterion};
use image::{GrayImage, Luma, DynamicImage};
use cullcore::metrics::blur::blur_score;
use cullcore::metrics::exposure::exposure_score;
use cullcore::scorer::score_images;

fn bench_blur(c: &mut Criterion) {
    let mut img = GrayImage::new(1000, 1000);
    for (x, y, p) in img.enumerate_pixels_mut() {
        let v = if (x + y) % 2 == 0 { 0 } else { 255 };
        *p = Luma([v]);
    }

    let img = DynamicImage::ImageLuma8(img);

    c.bench_function("blur_score_1mp", |b| {
        b.iter(|| blur_score(&img))
    });
}

fn bench_exposure(c: &mut Criterion) {
    let mut img = GrayImage::new(1000, 1000);
    for p in img.pixels_mut() {
        *p = Luma([128]);
    }

    let img = DynamicImage::ImageLuma8(img);

    c.bench_function("exposure_score_1mp", |b| {
        b.iter(|| exposure_score(&img))
    });
}

fn bench_full_scoring(c: &mut Criterion) {
    let mut images = Vec::new();

    for _ in 0..50 {
        let mut img = GrayImage::new(500, 500);
        for (x, y, p) in img.enumerate_pixels_mut() {
            let v = if (x + y) % 2 == 0 { 120 } else { 135 };
            *p = Luma([v]);
        }
        images.push(DynamicImage::ImageLuma8(img));
    }

    c.bench_function("score_50_images_parallel", |b| {
        b.iter(|| score_images(&images))
    });
}

criterion_group!(
    benches,
    bench_blur,
    bench_exposure,
    bench_full_scoring
);
criterion_main!(benches);
