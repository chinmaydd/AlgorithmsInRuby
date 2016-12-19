use std::fs::File;
use std::io::Read;

pub fn read_into_string(filepath: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(filepath).expect("Unable to open required file.");
    f.read_to_string(&mut input).expect("Unable to read string.");

    // Removing trailing newline.
    input.pop().expect("Could not remove trailing newline.");

    input
}
