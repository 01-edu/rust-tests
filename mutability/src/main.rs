// Write a function called doubtful that add to every passed to it a ?

// You have to fix the code to make it compile an for that you can
// only modify the code where is shown
#[derive(Debug)]
struct User {
	username: String,
	email: String,
	sign_in_count: i64,
	active: bool,
}

fn main() {
	let mut s = String::from("Hello");

	println!("Before changing the string: {}", s);

	change_str(&mut s);

	println!("After changing the string: {}", s);

	let user1 = User {
		email: String::from("someone@something.com"),
		username: String::from("name"),
		active: true,
		sign_in_count: 1,
	};

	println!("{:?}", user1);

	let user2 = User {
		email: String::from("another@example.com"),
		username: String::from("anotherusername567"),
		..user1
	};

	println!("{:?}", user2);
}

fn change_str(s: &mut String) {
	s.push_str("!")
}
