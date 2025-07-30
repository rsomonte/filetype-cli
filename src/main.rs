use std::fs;
use std::process;
use clap::Parser;
use ufile_core::identify_from_bytes;

/// Simple CLI for ufile-core
#[derive(Parser)]
struct Cli {
    /// Path to the file to identify
    file: String,
}

fn main() {
    let args = Cli::parse();

    let bytes = match fs::read(&args.file) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}: ERROR: {}", args.file, e);
            process::exit(1);
        }
    };

    match identify_from_bytes(&bytes) {
        Some(info) => println!("{}: {}", args.file, info.description),
        None => println!("{}: unknown file type", args.file),
    }
}