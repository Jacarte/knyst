#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

// TODO, this will be removed after the demo
// to merge with main
fuzz_target!(|data: &[u8]| {
    println!("Hello from fuzz, data {:?}", data);
});
