use std::process::Command;

const MANIFEST_PATH: &str = "../../solutions/hello_rust/Cargo.toml";

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hello() {
		let out = Command::new("cargo")
			.arg("run")
			.arg("--manifest-path")
			.arg(MANIFEST_PATH)
			.output()
			.expect("Failed to execute command");

		assert_eq!(String::from_utf8(out.stdout).unwrap(), "Hello, Rust!\n");
	}
}
