// Write a function that tries to open a file and panics if the file
// doesn't exist
use std::fs::{self, File};

fn main() {
	open_file("file.txt");
}

fn open_file(s: &str) -> File {
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
}
