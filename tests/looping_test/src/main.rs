use rexpect::spawn;

const MANIFEST_PATH: &str = "../../solutions/looping/Cargo.toml";

fn set_up() -> escargot::CargoRun {
	let temp = assert_fs::TempDir::new().unwrap();
	let cmd = escargot::CargoBuild::new()
		.bin("looping")
		.current_release()
		.current_target()
		.manifest_path(MANIFEST_PATH)
		.target_dir(temp.path())
		.run()
		.unwrap();
	cmd
}

fn main() {
	let _p = set_up();
	println!("hello");
}

#[test]
fn test_correct_answer() {
	let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";

	let cmd = set_up();
	let mut p = spawn(&cmd.path().display().to_string(), Some(2000)).unwrap();
	p.exp_string(riddle).unwrap();
	p.send_line("The letter e").unwrap();
	p.exp_string("It took you 1 trials to get the right answer")
		.unwrap();
}

#[test]
fn test_more_than_one_fail() {
	let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";

	let cmd = set_up();
	let mut p = spawn(&cmd.path().display().to_string(), Some(2000)).unwrap();
	p.exp_string(riddle).unwrap();
	p.send_line("circle").unwrap();
	p.exp_string(riddle).unwrap();
	p.send_line("relativity").unwrap();
	p.exp_string(riddle).unwrap();
	p.send_line("the big bang").unwrap();
	p.exp_string(riddle).unwrap();
	p.send_line("The letter e").unwrap();
	p.exp_string("It took you 4 trials to get the right answer")
		.unwrap();
}
