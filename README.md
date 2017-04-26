# enctool
Utility for checking character encodings in files and streams of text.

[![Travis](https://img.shields.io/travis/rust-lang/rust.svg)](https://travis-ci.org/Widen/enctool)

## Installation
enctool can be installed from source using the provided Makefile:

    make install

To create a distribution archive, run:

    make dist

## Usage
enctool can be given various commands to perform encoding checks and conversions. Usually the syntax is

    enctool [--command-flag] [-f FILE]

where `--command-flag` is a flag that tells enctool what process to perform. If the file is specified, it reads input from the given file. Otherwise it reads input from stdin.

See `enctool --help` for more details about usage.

## License
MIT
