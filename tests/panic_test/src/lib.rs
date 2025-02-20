use panic::*;
use std::fs::{self, File};

#[test]
#[should_panic]
fn test_opening() {
    open_file("file.txt");
}

#[test]
fn test_opening_existing() {
    let filename = "created.txt";
    File::create(filename).unwrap();
    open_file(filename);
    fs::remove_file(filename).unwrap();
}
