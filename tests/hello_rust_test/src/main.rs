use std::process::Command;

const MANIFEST_PATH: &str = "../../solutions/hello_rust/Cargo.toml";

fn main() {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hello() {
		let out = Command::new("cargo")
			.arg("run")
			.arg("--target-dir")
			.arg("/jail/tmp")
			.arg("--manifest-path")
			.arg(MANIFEST_PATH)
			.output()
			.expect("Failed to execute command");

		println!("{:?}", String::from_utf8(out.stderr));
		assert!(out.status.success());
		assert_eq!(String::from_utf8(out.stdout).unwrap(), "Hello, Rust!!\n");
	}
}
