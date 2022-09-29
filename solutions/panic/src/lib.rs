/*
## error types

### Instructions

Write a function that tries to open a file and panics if the file
doesn't exist

*/

use std::fs::File;

pub fn open_file(s: &str) -> File {
    let file = File::open(s);
    if let Ok(f) = file {
        return f;
    } else {
        panic!("File not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    #[should_panic(expected = "File not found")]
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
}
