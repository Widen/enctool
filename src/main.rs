extern crate encoding;
extern crate getopts;

use encoding::types::*;
use getopts::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::str;


fn main() {
    let mut options = Options::new();

    options.optflag("V", "validate", "Verify the file is valid in the given encoding.");
    options.optflag("", "check-utf8mb4", "Check for 4-byte characters in a UTF-8 file.");
    options.optopt("c", "convert-to", "Convert the input to a given output encoding.", "ENCODING");
    options.optopt("e", "encoding", "Specify the input encoding. Defaults to UTF-8.", "ENCODING");
    options.optopt("i", "input", "A file to parse, otherwise stdin is used.", "FILE");
    options.optopt("o", "output", "File to write any converted output to.", "FILE");
    options.optflag("l", "list", "List all supported encodings.");
    options.optflag("h", "help", "Show this help message.");
    options.optflag("v", "version", "Show the program version.");

    let matches = match options.parse(env::args()) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("help") {
        let short = options.short_usage("enctool");
        println!("Widen encoding tool\n\n{}", options.usage(&short));
        return;
    }

    if matches.opt_present("version") {
        println!("enctool {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if matches.opt_present("list") {
        for encoding in encoding::all::encodings() {
            if let Some(name) = encoding.whatwg_name() {
                println!("{}", name);
            }
        }
        return;
    }

    // Get the input stream.
    let mut input: Box<Read> = match matches.opt_str("input") {
        Some(filename) => Box::new(File::open(filename).expect("given file is not readable")),
        None => Box::new(io::stdin()),
    };

    // Get the encoding to use.
    let encoding = match matches.opt_str("encoding") {
        Some(name) => {
            match get_encoding(&name) {
                Some(encoding) => encoding,
                None => {
                    println!("Unknown encoding: {}", name);
                    return;
                },
            }
        },
        None => encoding::all::UTF_8,
    };

    if matches.opt_present("validate") {
        validate(&mut input, encoding);
        return;
    }

    if matches.opt_present("check-utf8mb4") {
        check_utf8mb4(&mut input);
        return;
    }

    // Get the encoding to use.
    if let Some(name) = matches.opt_str("convert-to") {
        let to_encoding = match get_encoding(&name) {
            Some(encoding) => encoding,
            None => {
                println!("Unknown encoding: {}", name);
                return;
            },
        };

        let mut output: Box<Write> = match matches.opt_str("output") {
            Some(filename) => Box::new(File::create(filename).expect("given file is not writable")),
            None => Box::new(io::stdout()),
        };

        convert(&mut input, encoding, &mut output, to_encoding);
        return;
    }

    println!("No command given.");
}

/// Validate an input stream against an encoding.
fn validate(reader: &mut Read, encoding: &Encoding) {
    let encoding_name = encoding.whatwg_name().unwrap_or("");
    let mut reader = BufReader::new(reader);
    let mut line = 1;
    let mut found = 0;

    loop {
        let mut buf = Vec::new();

        if reader.read_until(b'\n', &mut buf).unwrap() == 0 {
            break;
        }

        // Validate the line using strict decoding.
        if encoding.decode(&buf, DecoderTrap::Strict).is_err() {
            found += 1;

            // Lossily convert for display purposes.
            let mut lossy = encoding.decode(&buf, DecoderTrap::Replace).unwrap();
            if !lossy.ends_with("\n") {
                lossy.push('\n');
            }

            print!("line {}: invalid text: {}", line, lossy);
        }

        line += 1;
    }

    println!("Found {} lines containing invalid text for encoding {}.", found, encoding_name);
}

/// Convert an input stream from an encoding to an output stream in a different encoding.
fn convert(reader: &mut Read, src: &Encoding, writer: &mut Write, dest: &Encoding) {
    let mut reader = BufReader::new(reader);
    let mut line = 1;

    loop {
        // Read a line from the input.
        let mut src_buf = Vec::new();
        if reader.read_until(b'\n', &mut src_buf).unwrap() == 0 {
            break;
        }

        // Decode the line using the src encoding.
        let text = match src.decode(&src_buf, DecoderTrap::Strict) {
            Ok(string) => string,
            Err(e) => {
                println!("line {}: input error: {}", line, e);
                break;
            },
        };

        // Encode the line using the dest encoding.
        let dest_buf = match dest.encode(&text, EncoderTrap::Strict) {
            Ok(buf) => buf,
            Err(e) => {
                println!("line {}: output error: {}", line, e);
                break;
            },
        };

        // Write the converted bytes to the output.
        if let Err(e) = writer.write_all(&dest_buf) {
            println!("line {}: write error: {}", line, e);
            break;
        }

        line += 1;
    }

    println!("Converted {} lines.", line);
}

fn check_utf8mb4(reader: &mut Read) {
    let reader = BufReader::new(reader);
    let mut found = 0;

    for line in reader.lines() {
        for char in line.unwrap().chars() {
            let len = char.len_utf8();
            if len >= 4 {
                found += 1;
                println!("Found {}-byte UTF-8 character: {}", len, char);
            }
        }
    }

    println!("Found {} UTF-8 characters that are 4 bytes wide.", found);
}

fn get_encoding<S: AsRef<str>>(name: S) -> Option<EncodingRef> {
    let name = name.as_ref().to_lowercase();

    match name.as_str() {
        "utf16" => Some(encoding::all::UTF_16LE),
        _ => encoding::label::encoding_from_whatwg_label(&name),
    }
}
