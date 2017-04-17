extern crate encoding;
extern crate getopts;

use encoding::types::*;
use getopts::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, stdin};
use std::str;


fn main() {
    let mut options = Options::new();

    options.optflag("v", "validate", "Verify the file is valid in the given encoding.");
    options.optflag("", "check-utf8mb4", "Check for 4-byte characters in a UTF-8 file.");
    options.optopt("e", "encoding", "Specify the file encoding. Defaults to UTF-8.", "ENCODING");
    options.optopt("f", "file", "A file to parse, otherwise stdin is used.", "FILE");
    options.optflag("h", "help", "Show this help message.");

    let matches = match options.parse(env::args()) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("h") {
        let short = options.short_usage("enctool");
        println!("Widen encoding tool\n\n{}", options.usage(&short));
        return;
    }

    // Get the input stream.
    let mut input: Box<Read> = match matches.opt_str("f") {
        Some(filename) => Box::new(File::open(filename).expect("given file is not readable")),
        None => Box::new(stdin()),
    };

    // Get the encoding to use.
    let encoding = match matches.opt_str("e").map(|s| s.to_lowercase()).as_ref().map(|s| s as &str) {
        Some("utf16") => encoding::all::UTF_16LE,
        Some(name) => {
            match encoding::label::encoding_from_whatwg_label(name) {
                Some(encoding) => encoding,
                None => {
                    println!("Unknown encoding: {}", name);
                    return;
                },
            }
        },
        None => encoding::all::UTF_8,
    };

    if matches.opt_present("v") {
        validate(&mut input, encoding);
        return;
    }

    if matches.opt_present("check-utf8mb4") {
        check_utf8mb4(&mut input);
        return;
    }

    println!("No command given.");
}

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

        line += 1;

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
    }

    println!("Found {} lines containing invalid text for encoding {}.", found, encoding_name);
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
