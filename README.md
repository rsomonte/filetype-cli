# ufile-cli: Command-Line File Type Identification

`ufile-cli` is a fast and user-friendly command-line tool for identifying file types based on their binary signatures ("magic numbers"). It is powered by [`ufile-core`](https://github.com/rsomonte/ufile-core), a high-performance Rust library inspired by the classic Unix `file` command.

## How It Works

This CLI reads the first bytes of a file and uses `ufile-core` to match them against a database of known file signatures, providing a human-readable description of the detected file type.

## Usage

Build the project with Cargo:

```sh
cargo build --release
```

Run the CLI with one or more file paths or directories:

```sh
# Single file
cargo run -- path/to/your/file

# Multiple files
cargo run -- file1.txt file2.jpg file3.pdf

# Directory (processes all files in the directory)
cargo run -- path/to/directory/

# Mix of files and directories
cargo run -- file1.txt path/to/directory/ file2.png
```

Or, after building:

```sh
# Single file
./target/release/ufile-cli path/to/your/file

# Multiple files
./target/release/ufile-cli file1.txt file2.jpg file3.pdf

# Directory
./target/release/ufile-cli path/to/directory/

# Mix of files and directories
./target/release/ufile-cli file1.txt path/to/directory/ file2.png
```

Example output:

```
example.png: PNG image
document.pdf: PDF document
photo.jpg: JPEG image data
unknown.xyz: unknown file type
```

## Arguments

- `<FILES>...`: One or more paths to files or directories to identify. When a directory is provided, all files within that directory will be processed (non-recursively).

## Relationship to ufile-core

This CLI is a thin wrapper around [`ufile-core`](https://github.com/rsomonte/ufile-core), ensuring robust and consistent file type detection logic. By depending directly on the core library, `ufile-cli` always benefits from the latest improvements and signature updates.