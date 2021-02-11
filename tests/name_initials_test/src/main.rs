/*
## name_initials

### Instructions

Create a function called `initials`, this function will receive a vector of string literals
with names and return a vector of Strings with the initials of each name.

### Example:

```rust
```

> This exercise will test the **heap allocation** of your function!
> So try your best to allocate the minimum data on the heap!

### Notions

- https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html

*/

use assert_no_alloc::*;
#[allow(unused_imports)]
use name_initials::*;

#[global_allocator]
static A: AllocDisabler = AllocDisabler;

#[allow(dead_code)]
fn main() {
	let mut names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
	println!("{:?}", initials(&mut names));
	// output: ["H. P.", "S. E.", "J. L.", "B. O."]
}

#[allow(dead_code)]
struct Test<'a> {
	names: Vec<&'a str>,
	result: Vec<&'a str>,
}

// solution that will run against the students solution
// this function uses the less heap allocation
#[allow(dead_code)]
fn initials_sol(arr: &mut Vec<&str>) -> Vec<String> {
	arr.iter()
		.map(|ele| {
			let mut names = ele.split_whitespace();
			let mut a = names.next().unwrap().chars().nth(0).unwrap().to_string();
			a.push_str(". ");
			let mut b = names.next().unwrap().chars().nth(0).unwrap().to_string();
			b.push_str(".");
			a.push_str(&b);
			a
		})
		.collect()
}

#[test]
fn test_memory_allocation() {
	let mut test_value = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
	assert_no_alloc(|| initials_sol(&mut test_value));
	let sol_violations = violation_count();
	assert_no_alloc(|| initials(&mut test_value));
	let stu_violations = violation_count() - sol_violations;
	assert!(
		stu_violations <= sol_violations,
		format!(
			"You are allocating to the heap {} time, and it must be less or equal to {} times",
			stu_violations, sol_violations
		)
	);
}

#[test]
fn test_function() {
	let cases = vec![
		Test {
			names: vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"],
			result: vec!["H. P.", "S. E.", "J. L.", "B. O."],
		},
		Test {
			names: vec![
				"James John",
				"David Joseph",
				"Matthew Brian",
				"Jacob Sousa",
				"Bruce Banner",
				"Scarlett Johansson",
				"Graydon Hoare",
			],
			result: vec![
				"J. J.", "D. J.", "M. B.", "J. S.", "B. B.", "S. J.", "G. H.",
			],
		},
	];

	for mut v in cases {
		assert_eq!(
			initials(&mut v.names),
			v.result
				.iter()
				.map(|ele| ele.to_string())
				.collect::<Vec<String>>()
		);
	}
}
