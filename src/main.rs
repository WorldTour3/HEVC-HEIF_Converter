
use clap::Parser;
use hevc_converter::{convert_to_jpeg, convert_to_mp4, get_output_path};
use rayon::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

/// A tool to convert HEIF/HEVC media to JPEG/MP4
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file or directory
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let output_dir = args.output.unwrap_or_else(|| {
        args.input.parent().map_or_else(|| args.input.clone(), |p| p.to_path_buf())
    });

    WalkDir::new(&args.input)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .par_bridge()
        .for_each(|entry| {
            let input_path = entry.path();
            let extension = input_path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

            let output_path = get_output_path(input_path, &output_dir);

            let result = match extension.as_str() {
                "heic" | "heif" => convert_to_jpeg(input_path, &output_path),
                "mov" | "mp4" => convert_to_mp4(input_path, &output_path),
                _ => return,
            };

            if let Err(e) = result {
                eprintln!("Failed to convert {:?}: {}", input_path, e);
            }
        });
}
