use crate::error::CullError;
use image::{DynamicImage, ImageFormat};
use std::path::{Path, PathBuf};

pub struct LoadedImage {
    pub path: PathBuf,
    pub image: DynamicImage,
}

pub fn load_images_from_dir(dir: &Path) -> Result<Vec<LoadedImage>, CullError> {
    if !dir.exists() {
        return Err(CullError::InvalidPath(format!(
            "Path does not exist: {}",
            dir.display()
        )));
    }

    if !dir.is_dir() {
        return Err(CullError::InvalidPath(format!(
            "Path is not a directory: {}",
            dir.display()
        )));
    }

    let entries = std::fs::read_dir(dir).map_err(|e| CullError::InvalidPath(e.to_string()))?;

    let mut images = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| CullError::InvalidPath(e.to_string()))?;
        let path = entry.path();

        if !is_supported(&path) {
            continue;
        }

        let img = image::open(&path).map_err(|e| CullError::ImageLoad(e.to_string()))?;

        images.push(LoadedImage { path, image: img });
    }

    Ok(images)
}

fn is_supported(path: &Path) -> bool {
    match image::guess_format(&std::fs::read(path).unwrap()) {
        Ok(ImageFormat::Jpeg | ImageFormat::Png) => true,
        _ => false,
    }
}
