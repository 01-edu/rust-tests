/*
## name_initials

### Instructions

Create a function called `initials`, this function will receive a vector of string literals
with names and return a vector of Strings with the initials of each name.

### Example:

```rust
fn main() {
	let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"]
	println!("{:?}", initials(names));
	// output: ["H. P.", "S. E.", "J. L.", "B. O."]
}
```

> This exercise will test the **heap allocation** of your function!
> So try your best to allocate the minimum data on the heap!

### Notions

- https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html

*/

pub fn initials(names: &mut Vec<&str>) -> Vec<String> {
	names
		.iter()
		.map(|x| {
			let mut two_names = x.split_whitespace();
			let mut a = two_names
				.next()
				.unwrap()
				.chars()
				.nth(0)
				.unwrap()
				.to_string();
			a.push_str(". ");
			a.push_str(
				&two_names
					.next()
					.unwrap()
					.chars()
					.nth(0)
					.unwrap()
					.to_string(),
			);
			a.push_str(".");
			a
		})
		.collect()
}
