
use jpeg_encoder::{ColorType, Encoder, EncodingError};
use libheif_rs::{ColorSpace, HeifContext, HeifError, LibHeif, RgbChroma};
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HEIF decoding error: {0}")]
    Heif(#[from] HeifError),
    #[error("JPEG encoding error: {0}")]
    Jpeg(#[from] EncodingError),
    #[error("FFMPEG command failed: {0}")]
    Ffmpeg(String),
    #[error("File name not found for: {0}")]
    FileNameNotFound(PathBuf),
}

type Result<T> = std::result::Result<T, ConversionError>;

pub fn get_output_path(input_path: &Path, output_dir: &Path, extension: &str) -> PathBuf {
    let file_stem = input_path.file_stem().unwrap().to_str().unwrap();
    let new_extension = match extension {
        "heic" | "heif" => "jpg",
        "mov" | "mp4" => "mp4",
        _ => "mp4",
    };
    output_dir.join(format!("{}.{}", file_stem, new_extension))
}

pub fn convert_to_jpeg(input_path: &Path, output_path: &Path) -> Result<()> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file(input_path.to_str().unwrap())?;
    let handle = ctx.primary_image_handle()?;
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    let planes = image.planes().interleaved.unwrap();

    let encoder = Encoder::new_file(output_path, 95)?;
    encoder.encode(
        planes.data,
        planes.width as u16,
        planes.height as u16,
        ColorType::Rgb,
    )?;
    println!("Converted {:?} to {:?}", input_path, output_path);
    Ok(())
}

pub fn convert_to_mp4(input_path: &Path, output_path: &Path) -> Result<()> {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c:v")
        .arg("libx264")
        .arg("-c:a")
        .arg("aac")
        .arg(output_path)
        .output()?;

    if !output.status.success() {
        return Err(ConversionError::Ffmpeg(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    println!("Converted {:?} to {:?}", input_path, output_path);
    Ok(())
}
