// Write a function called doubtful that add to every passed to it a ?

// You have to fix the code to make it compile an for that you can
// only modify the code where is shown

fn main() {
	let mut s = String::from("Hello");
	println!("Before changing the string: {}", s);
	change_str(&mut s);
	println!("After changing the string: {}", s);
}

fn change_str(s: &mut String) {
	s.push_str("!")
}
