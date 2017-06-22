use std::ffi::CStr;
use std::io::Read;
use std::str::from_utf8;
use uchardet_sys::*;


/// Guess the encoding of an input stream.
pub fn guess(reader: &mut Read) -> Option<String> {
    let uchardet = unsafe {
        uchardet_new()
    };

    let mut buffer = [0; 8192];
    while let Ok(len) = reader.read(&mut buffer) {
        if len == 0 {
            break;
        }

        unsafe {
            if uchardet_handle_data(uchardet, buffer.as_ptr() as *const i8, len) > 0 {
                break;
            }
        }
    }

    let encoding = unsafe {
        uchardet_data_end(uchardet);

        let bytes = CStr::from_ptr(uchardet_get_charset(uchardet)).to_bytes();
        match from_utf8(bytes) {
            Err(_) => None,
            Ok("") => None,
            Ok(name) => Some(name.to_string()),
        }
    };

    unsafe {
        uchardet_delete(uchardet);
    }

    encoding
}
