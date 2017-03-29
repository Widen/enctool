# enctool
Utility for checking character encodings in files and streams of text.

## Compiling
enctool is written in Rust and can be built using Cargo.

    cargo build

## Usage
enctool can be given various commands to perform encoding checks and conversions. Ususally the syntax is

    enctool [--command-flag] [-f FILE]

where `--command-flag` is a flag that tells enctool what process to perform. If the file is specified, it reads input from the given file. Otherwise it reads input from stdin.

See `enctool --help` for more details about usage.

## License
MIT
