# filetype-cli: Command-Line File Type Identification

`filetype-cli` is a fast and user-friendly command-line tool for identifying file types based on their binary signatures ("magic numbers"). It is powered by [`filetype-core`](https://github.com/rsomonte/filetype-core), a high-performance Rust library inspired by the classic Unix `file` command.

## How It Works

This CLI reads the first bytes of a file and uses `filetype-core` to match them against a database of known file signatures, providing a human-readable description of the detected file type.

## Usage

Build the project with Cargo:

```sh
cargo build --release
```

Run the CLI with a file path:

```sh
cargo run -- path/to/your/file
```

Or, after building:

```sh
./target/release/filetype-cli path/to/your/file
```

Example output:

```
example.png: PNG image
```

If the file type is unknown:

```
example.unknown: unknown file type
```

## Arguments

- `file`: Path to the file to identify

## Relationship to filetype-core

This CLI is a thin wrapper around [`filetype-core`](https://github.com/rsomonte/filetype-core), ensuring robust and consistent file type detection logic. By depending directly on the core library, `filetype-cli` always benefits from the latest improvements and signature updates.