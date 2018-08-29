extern crate afl;
extern crate libflate;

use libflate::zlib::Decoder;
use std::io::Read;

// Suppress false positives on Address Sanitizer
const ASAN_DEFAULT_OPTIONS: &'static [u8] = b"detect_odr_violation=1\0";
#[no_mangle]
pub extern "C" fn __asan_default_options() -> *const u8 {
    ASAN_DEFAULT_OPTIONS as *const [u8] as *const u8
}

fn main() {
    afl::read_stdio_bytes(|data| {
        let decoder = Decoder::new(data.as_slice());
        if let Ok(mut contents) = decoder {
            let mut buffer = Vec::with_capacity(4096);
            contents.read_to_end(&mut buffer);
        }
    });
}