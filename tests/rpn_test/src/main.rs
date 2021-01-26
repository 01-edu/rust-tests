fn main() {}

#[cfg(test)]
mod tests {
    use std::process::Command;

    const MANIFEST_PATH: &str = "../../solutions/rpn/Cargo.toml";

    fn run(s: &str) -> String {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--manifest-path")
            .arg(MANIFEST_PATH)
            .arg(s)
            .output()
            .expect("Failed to execute command");

        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    #[test]
    fn error_tests() {
        assert_eq!("Error", run("21 3 2 % 2 3 2 *"));
        assert_eq!("Error", run("1 2 3 4 +"));
        assert_eq!("Error", run("324   +    1 - 23 "));
        assert_eq!("Error", run("32   / 22"));
        assert_eq!("Error", run("88 67 dks -"));
    }

    #[test]
    fn simple_tests() {
        assert_eq!("33", run("11 22 +"));
        assert_eq!("72", run("11016 153 /"));
        assert_eq!("1140", run("15 76 *"));
        assert_eq!("-78539698", run("23491234 102030932 -"));
    }

    #[test]
    fn complex_tests() {
        assert_eq!("10", run("1 2 * 3 * 4 +"));
        assert_eq!("2", run("3 1 2 * * 4 %"));
        assert_eq!("0", run("5 10 2 / - 50 *"));
    }

    #[test]
    fn with_spaces() {
        assert_eq!("44", run("299   255 %"));
        assert_eq!("1", run("     1      3 * 2 -"));
    }
}
