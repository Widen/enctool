use chardet::UniversalDetector;
use std::io::Read;


/// Guess the encoding of an input stream.
pub fn guess(reader: &mut Read) -> Option<String> {
    let mut detector = UniversalDetector::new();

    let mut buffer = [0; 8192];
    while let Ok(len) = reader.read(&mut buffer) {
        if len == 0 {
            break;
        }

        detector.feed(&buffer);
    }

    let result = detector.close();

    if result.0.is_empty() {
        None
    } else {
        Some(result.0)
    }
}
