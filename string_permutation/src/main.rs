// Define the function `is_permutation` that returns true if the
// string `s1` is a permutation of `s2`, otherwise it returns false
// `s1` is a permutation of `s2` if all the elements in `s1` appear the
// same number of times in `s2` and all the characters in `s1` appear in
// `s2` even if in different order)

fn main() {
	let word = "thought";
	let word1 = "thougth";
	println!(
		"Is `{}` a permutation of `{}`? = {}",
		word,
		word1,
		is_permutation(word, word1)
	);
}

use std::collections::HashMap;

fn is_permutation(s1: &str, s2: &str) -> bool {
	let mut s1_rep = HashMap::new();
	let mut s2_rep = HashMap::new();
	for c1 in s1.chars() {
		let repetitions = s1_rep.entry(c1).or_insert(1);
		*repetitions += 1;
	}
	for c2 in s2.chars() {
		let repetitions = s2_rep.entry(c2).or_insert(1);
		*repetitions += 1;
	}

	s1_rep == s2_rep
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn permutation_ascii() {
		assert!(is_permutation("abcde", "edbca"));
		assert!(!is_permutation("avcde", "edbca"));
		assert!(!is_permutation("cde", "edbca"));
		assert!(is_permutation("code", "deco"));
		assert!(!is_permutation("code", "deeco"));
		assert!(!is_permutation("codde", "deeco"));
	}

	#[test]
	fn permutation_unicode() {
		assert!(is_permutation("hello♥", "♥oelhl"));
		assert!(!is_permutation("♥", "♥♥"));
	}
}
