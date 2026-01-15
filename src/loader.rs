use std::path::{Path, PathBuf};
use image::{DynamicImage, ImageFormat};
use crate::error::CullError;

pub struct LoadedImage {
    pub path: PathBuf,
    pub image: DynamicImage,
}

pub fn load_images_from_dir(dir: &Path) -> Result<Vec<LoadedImage>, CullError> {
    if !dir.is_dir() {
        return Err(CullError::InvalidPath);
    }

    let mut images = Vec::new();

    for entry in std::fs::read_dir(dir).map_err(|_| CullError::InvalidPath)? {
        let entry = entry.map_err(|_| CullError::InvalidPath)?;
        let path = entry.path();

        if !is_supported(&path) {
            return Err(CullError::UnsupportedFormat);
        }

        let img = image::open(&path)
            .map_err(|e| CullError::ImageLoad(e.to_string()))?;

        images.push(LoadedImage {
            path,
            image: img,
        });
    }

    Ok(images)
}


fn is_supported(path: &Path) -> bool {
    match image::guess_format(
        &std::fs::read(path).unwrap()
    ) {
        Ok(ImageFormat::Jpeg | ImageFormat::Png) => true,
        _ => false,
    }
}
