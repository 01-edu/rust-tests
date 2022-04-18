// Given a list of integers (Vec<i32>) write three functions
// Write a function called `mean` that calculates the `mean` (the average value) of all the values in the list
// Write a function called `median` that calculates the `median` (for a sorted list is the value in the middle)
// Write a function called `mode` that calculates the mode (the value
// that appears more often)

use std::collections::HashMap;

// fn main() {
// 	println!("Hello, world!");
// 	let v = vec![4, 7, 5, 2, 5, 1, 3];
// 	println!("mean {}", mean(&v));
// 	println!("median {}", median(&v));
// 	println!("mode {}", mode(&v));
// }

pub fn mean(list: &Vec<i32>) -> f64 {
	let sum: i32 = list.iter().sum();
	sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
	let mut ordered = list.clone();
	ordered.sort();

	if list.len() % 2 == 0 {
		return (ordered[ordered.len() / 2] + ordered[ordered.len() / 2 - 1]) / 2;
	} else {
		return ordered[ordered.len() / 2];
	}
}

pub fn mode(list: &Vec<i32>) -> i32 {
	let mut mode = list[0];
	let mut occurrences: HashMap<i32, u32> = HashMap::new();
	for v in list.iter() {
		let count = occurrences.entry(*v).or_insert(0);
		*count += 1;
	}

	let mut max: u32 = 0;

	for (key, value) in occurrences {
		if value > max {
			mode = key;
			max = value;
		}
	}
	mode
}

#[cfg(test)]
mod test {
	use super::*;
	use std::f64;

	fn approx_eq(a: f64, b: f64) -> bool {
		(a - b).abs() < f64::EPSILON
	}

	#[test]
	fn test_mean() {
		let v = vec![4, 7, 5, 2, 5, 1, 3];
		assert!(approx_eq(mean(&v), 3.857142857142857));
	}

	#[test]
	fn test_median() {
		let v = vec![4, 7, 5, 2, 5, 1, 3];
		assert_eq!(median(&v), 4);

		let aux1 = vec![3, 1];
		assert_eq!(median(&aux1), 2);

		let aux2 = vec![1, 6, 5, 4];
		assert_eq!(median(&aux2), 4);
	}

	#[test]
	fn test_mode() {
		let v = vec![4, 7, 5, 2, 5, 1, 3];
		assert_eq!(mode(&v), 5);
	}
}
