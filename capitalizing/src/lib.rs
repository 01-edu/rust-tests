// Complete the `capitalize_first` function that turns the first letter of a string uppercase.
// Complete the `title_case` function that turns the first letter of each word in a string uppercase.
// Complete the `change_case` function that turns the uppercase letters of a string into lowercase and
// the lowercase letters into uppercase.

pub fn capitalize_first(input: &str) -> String {
	let mut c = input.chars();
	match c.next() {
		None => String::new(),
		Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
	}
}

pub fn title_case(input: &str) -> String {
	let words = input
		.split(" ")
		.map(|w| capitalize_first(w))
		.collect::<Vec<String>>();
	words.join(" ")
}

pub fn change_case(input: &str) -> String {
	input
		.chars()
		.map(|c: char| -> char {
			if c.is_lowercase() {
				c.to_uppercase().nth(0).unwrap()
			} else {
				c.to_lowercase().nth(0).unwrap()
			}
		})
		.collect::<String>()
}

// fn main() {
//     println!("{}", capitalize_first("joe is missing"));
//     println!("{:?}", title_case("jill is leaving A"));
//     println!("{}",change_case("heLLo THere"));
// }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_success() {
		assert_eq!(capitalize_first("hello"), "Hello");
		assert_eq!(capitalize_first("this is working"), "This is working");
	}

	#[test]
	fn test_titlle_case() {
		assert_eq!(title_case("this is a tittle"), "This Is A Tittle");
		assert_eq!(title_case("hello my name is carl"), "Hello My Name Is Carl");
	}

	#[test]
	fn test_change_case() {
		assert_eq!(change_case("PROgraming"), "proGRAMING");
		assert_eq!(change_case("heLLo THere"), "HEllO thERE");
	}

	#[test]
	fn test_empty() {
		assert_eq!(capitalize_first(""), "");
		assert_eq!(title_case(""), "");
		assert_eq!(change_case(""), "");
	}
}
