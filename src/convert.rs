use encoding::types::*;
use std::io::{BufRead, BufReader, Read, Write};
use std::str;


/// Convert an input stream from an encoding to an output stream in a different encoding.
pub fn convert(reader: &mut Read, src: &Encoding, writer: &mut Write, dest: &Encoding) {
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
