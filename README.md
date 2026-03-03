# HEVC/HEIF Converter

A simple Rust-based command-line tool to batch convert HEVC and HEIF files to MP4 and JPEG formats.

## Features

-   Converts HEIF (.heic, .heif) images to JPEG (.jpg).
-   Converts HEVC (.mov, .mp4) videos to universally compatible MP4 (H.264/AAC).
-   Processes single files or entire directories.
-   Uses multithreading to process files in parallel.

## Project Structure

```
hevc-converter/
├── Cargo.toml      # Project manifest and dependencies
├── README.md       # This file
└── src/
    ├── lib.rs      # Core conversion logic (HEIF→JPEG, HEVC→MP4)
    └── main.rs     # CLI entry point and argument parsing
```

## Prerequisites

You must have the following external dependencies installed on your system for this tool to work correctly.

-   **FFmpeg**: Required for video conversion.
    -   **macOS (Homebrew)**: `brew install ffmpeg`
    -   **Ubuntu/Debian**: `sudo apt update && sudo apt install ffmpeg`
-   **libheif**: Required for image conversion.
    -   **macOS (Homebrew)**: `brew install libheif`
    -   **Ubuntu/Debian**: `sudo apt update && sudo apt install libheif-dev`

## Installation

1.  Clone this repository:
    ```bash
    git clone <repository_url>
    cd hevc-converter
    ```

2.  Build the project in release mode:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/hevc-converter`.

## Usage

You can run the tool directly with `cargo run` or use the compiled binary.

### Command-Line Arguments

-   `-i, --input <PATH>`: The path to the input file or directory to process.
-   `-o, --output <PATH>`: (Optional) The path to the output directory where converted files will be saved. If not provided, files are saved in the same directory as the input.

### Examples

1.  **Convert a single image:**
    ```bash
    ./target/release/hevc-converter -i /path/to/my/image.heic
    ```
    This will create `image.jpg` in the `/path/to/my/` directory.

2.  **Convert a single video and specify an output directory:**
    ```bash
    ./target/release/hevc-converter -i /path/to/my/video.mov -o /path/to/output
    ```
    This will create `video.mp4` in the `/path/to/output` directory.

3.  **Convert all supported files in a directory:**
    ```bash
    ./target/release/hevc-converter -i /path/to/my/media_folder
    ```
    The tool will scan the directory, convert all `.heic`, `.heif`, `.mov`, and `.mp4` files, and save the new files in the same directory.
