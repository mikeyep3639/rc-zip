#![no_main]

use libfuzzer_sys::fuzz_target;
use rc_zip::{self, prelude::ReadZip};

fuzz_target!(|input: &[u8]| {
    let x = input.read_zip();
});