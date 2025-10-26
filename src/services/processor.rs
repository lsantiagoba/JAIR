use crate::models::Size;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use image::imageops::FilterType;
use rayon::prelude::*;

pub fn resize_and_save(
    input: &Path,
    out_dir: &Path,
    sizes: &[Size],
    output_format_png: bool,
) -> Result<Vec<PathBuf>> {

    let img = image::open(input)
        .with_context(|| format!("No se pudo abrir la imagen: {}", input.display()))?;

    let mut saved = Vec::new();

    for s in sizes {

        let resized = img.resize(s.width, s.height, FilterType::Lanczos3);

        let folder = out_dir.join(&s.name);
        std::fs::create_dir_all(&folder)
            .with_context(|| format!("No se pudo crear directorio: {}", folder.display()))?;

        let filename = input.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("image");

        let out_path = if output_format_png {
            folder.join(format!("{}-{}.png", filename, s.name))
        } else {
            folder.join(format!("{}-{}.jpg", filename, s.name))
        };

        if output_format_png {
            resized
                .save_with_format(&out_path, image::ImageFormat::Png)
                .with_context(|| format!("Error guardando {}", out_path.display()))?;
        } else {

            let mut fout = std::fs::File::create(&out_path)?;
            let quality = 90u8;
            resized.write_to(&mut std::io::BufWriter::new(&mut fout), image::ImageOutputFormat::Jpeg(quality))
                .with_context(|| format!("Error guardando {}", out_path.display()))?;
        }

        saved.push(out_path);
    }

    Ok(saved)
}

/// Process multiple images in batch
pub fn batch_resize_and_save(
    inputs: &[PathBuf],
    out_dir: &Path,
    sizes: &[Size],
    output_format_png: bool,
) -> Result<Vec<(PathBuf, Result<Vec<PathBuf>>)>> {

    let results: Vec<(PathBuf, Result<Vec<PathBuf>>)> = inputs
        .iter()
        .map(|input| {
            let result = resize_and_save(input, out_dir, sizes, output_format_png);
            (input.clone(), result)
        })
        .collect();

    Ok(results)
}

/// Process multiple images in parallel for better performance
pub fn batch_resize_and_save_parallel(
    inputs: &[PathBuf],
    out_dir: &Path,
    sizes: &[Size],
    output_format_png: bool,
) -> Result<Vec<(PathBuf, Result<Vec<PathBuf>>)>> {

    let results: Vec<(PathBuf, Result<Vec<PathBuf>>)> = inputs
        .par_iter()
        .map(|input| {
            let result = resize_and_save(input, out_dir, sizes, output_format_png);
            (input.clone(), result)
        })
        .collect();

    Ok(results)
}

/// Validate if a file is a supported image format
pub fn is_supported_image(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        matches!(
            ext_lower.as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "ico" | "tiff" | "webp"
        )
    } else {
        false
    }
}

/// Get all supported image files from a directory
pub fn get_images_from_directory(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut images = Vec::new();

    if !dir.is_dir() {
        return Err(anyhow::anyhow!("Path is not a directory: {}", dir.display()));
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && is_supported_image(&path) {
            images.push(path);
        }
    }

    Ok(images)
}
