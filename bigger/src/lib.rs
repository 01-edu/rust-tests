// Create the function `bigger` that gets the biggest positive number in the `HashMap`

use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
	let mut max_val = 0;
	if max_val >= 0 {
		for (_, v) in h.iter() {
			if *v > max_val {
				max_val = *v;
			}
		}
	}
	return max_val;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_positive() {
		let mut f = HashMap::new();
		f.insert("Daniel", 12);
		f.insert("Ashley", 333);
		f.insert("Katie", 334);
		f.insert("Robert", 14);

		assert_eq!(334, bigger(f));
	}
	#[test]
	fn test_negative() {
		let mut f = HashMap::new();
		f.insert("Daniel", 41758712);
		f.insert("Ashley", 54551444);
		f.insert("Katie", 575556334);
		f.insert("Robert", 574148);

		assert_eq!(575556334, bigger(f));
	}
}
