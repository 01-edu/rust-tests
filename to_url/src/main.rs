// Define a function called `to_url` that takes a string and
// substitutes every white-space with '%20'

fn to_url(s: &str) -> String {
	let mut s_r = String::new();
	for c in s.chars() {
		if c == ' ' {
			s_r.push_str("%20");
			continue;
		}
		s_r.push(c);
	}
	s_r
}

fn main() {
	let s = "Hello, world!";
	println!("{} to be use as an url is {}", s, to_url(s));
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_url() {
		assert_eq!(to_url("this is my search"), "this%20is%20my%20search");
	}
}
