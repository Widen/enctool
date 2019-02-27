# enctool

[![Build Status](https://badge.buildkite.com/9d30549c2aeac3588dd40957549a346ebf15f5d237eedd6682.svg)](https://buildkite.com/widen/enctool)

Utility for checking character encodings in files and streams of text.

Made with :heart: by Widen.

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
