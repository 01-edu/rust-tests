use rand::{Rng, distr::Alphanumeric};
use std::process::{Command, Output};

const MANIFEST_PATH: &str = "../../solutions/brackets_matching/Cargo.toml";

#[inline]
fn run(s: Vec<&str>) -> Output {
    Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg(MANIFEST_PATH)
        .args(s.iter())
        .output()
        .expect("Failed to execute command")
}

#[test]
fn random_tests() {
    fn random_alnum() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect()
    }

    let mut args = vec![
        String::from("(johndoe)"),
        String::from("()"),
        String::from("([])"),
        String::from("{2*[d - 3]/(12)}"),
    ];

    for _ in 0..3 {
        args.push(format!("({:?})", &random_alnum()));
        args.push(format!("[{:?}]", &random_alnum()));
        args.push(format!("{}{:?}{}", "{", &random_alnum(), "}"));
    }

    for v in args.iter() {
        let output = run(vec![v]);
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "OK\n");
    }
}

#[test]
fn tests_both() {
    let arr = [
        (["", "{[(0 + 0)(1 + 1)](3*(-1)){()}}"], "OK\nOK\n"),
        (["{][]}", "{3*[21/(12+ 23)]}"], "Error\nOK\n"),
        (["{([)])}", "{{{something }- [something]}}"], "Error\nOK\n"),
    ];

    for t in arr.iter() {
        let output = run(t.0.to_vec());
        assert_eq!(String::from_utf8_lossy(&output.stdout), t.1);
    }
}

#[test]
fn tests_with_nothing() {
    let output = run(vec![]);
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
}
