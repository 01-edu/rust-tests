//use assert_cmd::prelude::*; // Add methods on commands
use assert_cmd::Command;

#[test]
fn test_right_answer_first_try() {
    let mut cmd = Command::cargo_bin("looping").unwrap();
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?\n";
    let _assert = cmd
        .timeout(std::time::Duration::from_millis(3))
        .write_stdin("The letter e\n")
        .assert()
        .stdout(riddle.to_string() + "Number of trials: 1\n");
}

#[test]
fn test_right_answer_after_the_first_try() {
    let mut cmd = Command::cargo_bin("looping").unwrap();
    let ref tried = [
        "circle",
        "relativity",
        "big bang",
        "water",
        "The letter e\n",
    ];
    let joined = tried.join("\n");
    let riddle ="I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?\n";
    let mut output = String::new();
    for _ in tried {
        output.push_str(riddle);
    }
    println!("{}", joined);

    let _assert = cmd
        .timeout(std::time::Duration::from_millis(2))
        .write_stdin(joined)
        .assert()
        .stdout(output + &format!("Number of trials: {}\n", tried.len()));
}
