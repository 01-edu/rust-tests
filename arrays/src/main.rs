// Write a function that takes an array of i32 and returns the sum of
// the elements
// Define a function that returns an array with 50 elements with the
// value 10

// Define a function call thirtytwo_tens that returns an array with 32
// positions fill with only the value 10: [10, 10, 10, ... 10].len()
// = 32
fn main() {
	let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	let a1: Vec<i32> = (1..11).collect();
	let b = [5; 10];

	println!("The Sum of the elements in {:?} = {}", a, sum(a));
	println!("The Sum of the elements in {:?} = {}", a, sum_v2(a));
	println!("The Sum of the elements in {:?} = ", a1);
	println!("The Sum of the elements in {:?} = {}", b, sum(b));
	println!(
		"Array size 50 with all elements to 10 = {:?}",
		thirtytwo_tens().len()
	);
}

fn sum(a: [i32; 10]) -> i32 {
	let mut result = 0;
	for e in a.iter() {
		result += e;
	}
	result
}

fn sum_v2(a: [i32; 10]) -> i32 {
	let mut result = 0;
	let mut i = 0;
	while i < a.len() {
		result += a[i];
		i += 1;
	}
	result
}

#[allow(dead_code)]
fn thirtytwo_tens() -> [i32; 32] {
	[10; 32]
}
