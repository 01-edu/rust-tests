// Write a function that tries to open a file and if it doesn't exist
// creates it and returns it
use std::fs::File;
use std::io::ErrorKind;

fn main() {
	open_or_create("hello.txt");
}

fn open_or_create(filename: &str) -> File {
	let file = File::open(filename);
	let f = match file {
		Ok(f) => f,
		Err(err) => match err.kind() {
			ErrorKind::NotFound => File::create(filename).unwrap(),
			other_error => panic!("Problem opening the file: {:?}", other_error),
		},
	};
	f
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_open_or_create() {
		open_or_create("hello.txt");
	}
}
