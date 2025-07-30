use std::fs;
use std::path::Path;
use clap::Parser;
use ufile_core::identify_from_bytes;

/// Simple CLI for ufile-core
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Paths to files or directories to identify
    #[arg(required = true, help = "Files or directories to analyze")]
    files: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    for file_path in &args.files {
        process_path(file_path);
    }
}

fn process_path(path: &str) {
    let path = Path::new(path);
    
    if path.is_dir() {
        process_directory(path);
    } else if path.is_file() {
        process_file(path);
    } else {
        eprintln!("{}: ERROR: No such file or directory", path.display());
    }
}

fn process_directory(dir_path: &Path) {
    let entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("{}: ERROR: {}", dir_path.display(), e);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                eprintln!("ERROR reading directory entry: {}", e);
                continue;
            }
        };

        let path = entry.path();
        if path.is_file() {
            process_file(&path);
        }
        // Skip subdirectories to avoid recursion for now
    }
}

fn process_file(file_path: &Path) {
    let bytes = match fs::read(file_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}: ERROR: {}", file_path.display(), e);
            return;
        }
    };

    match identify_from_bytes(&bytes) {
        Some(info) => println!("{}: {}", file_path.display(), info.description),
        None => println!("{}: unknown file type", file_path.display()),
    }
}