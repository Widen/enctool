use encoding::types::*;
use std::io::{BufRead, BufReader, Read};
use std::str;


/// Validate an input stream against an encoding.
pub fn validate(reader: &mut Read, encoding: &Encoding) {
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
