extern crate getopts;

use getopts::*;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, stdin};
use std::str;


fn main() {
    let mut options = Options::new();

    options.optflag("", "check-utf8", "Validate an UTF-8 file (default)");
    options.optflag("", "check-utf8mb4", "Check for 4-byte characters in a UTF-8 file");
    options.optopt("f", "file", "A file to parse, otherwise stdin is used", "FILE");
    options.optflag("h", "help", "Show this help message");

    let matches = match options.parse(env::args()) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("h") {
        let short = options.short_usage("enctool");
        println!("Widen encoding tool\n\n{}", options.usage(&short));
        return;
    }

    let mut input: Box<Read> = match matches.opt_str("f") {
        Some(filename) => Box::new(File::open(filename).expect("given file is not readable")),
        None => Box::new(stdin()),
    };

    if matches.opt_present("check-utf8mb4") {
        check_utf8mb4(&mut input);
        return;
    }

    check_utf8(&mut input);
}

fn check_utf8(reader: &mut Read) {
    let mut reader = BufReader::new(reader);
    let mut line = 1;
    let mut found = 0;

    loop {
        let mut buf = Vec::new();

        if reader.read_until(b'\n', &mut buf).unwrap() == 0 {
            break;
        }

        line += 1;

        if str::from_utf8(&buf).is_err() {
            found += 1;

            let mut lossy = String::from_utf8_lossy(&buf).into_owned();
            if !lossy.ends_with("\n") {
                lossy.push('\n');
            }

            print!("Invalid UTF-8 ({}): {}", line, lossy);
        }
    }

    println!("Found {} lines containing invalid UTF-8.", found);
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
