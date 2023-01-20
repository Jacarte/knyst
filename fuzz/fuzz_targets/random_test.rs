#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

// TODO, this will be removed after the demo
// to merge with main
fuzz_target!(|data: &[u8]| {
    println!("Hello from fuzz, data {:?}", data);

    // Crash if data len > 3
    //
    //if data.len() > 3 {
    //    panic!("Test crash 1");
    //}

    // More complex example
    if data.len() > 100 && data[2] == 200 {
        panic!("Test crash 2")
    }
});
