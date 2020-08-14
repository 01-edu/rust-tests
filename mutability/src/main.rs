// Write a function called doubtful that add to every string passed to it a ?

// You have to fix the code to make it compile and for that you can
// only modify the code where is indicated
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

	doubtful(&mut s);

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

fn doubtful(s: &mut String) {
	s.push_str("?")
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn it_works() {
		let mut s = "hello".to_string();
		let s_copy = s.clone();

		doubtful(&mut s);

		assert_eq!(s, s_copy + "?");
	}
}
