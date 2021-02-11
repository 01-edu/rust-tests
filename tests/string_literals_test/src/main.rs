/*
## string literals

### Instructions

Create the following functions:

- `is_empty`, that returns true if a string is empty
- `is_ascii`, that returns true if all characters of a given string is in ASCII range
- `contains`, that returns true if the string contains a pattern given
- `split_at`, that divides a string in two returning a tuple
- `find', that returns the index if the first character of a given string that matches the pattern

> This exercise will test the **heap allocation** of your function!
> So try your best to allocate the minimum data on the heap! (hit: &str)

### Notions

- https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html
- https://doc.rust-lang.org/rust-by-example/primitives/literals.html

*/
use assert_no_alloc::*;

#[global_allocator]
static A: AllocDisabler = AllocDisabler;

#[allow(unused_imports)]
use string_literals::*;

fn main() {
	println!("{}", is_empty(""));
	println!("{}", is_ascii("rust"));
	println!("{}", contains("rust", "ru"));
	println!("{:?}", split_at("rust", 2));
	println!("{}", find("rust", 'u'));
}

#[test]
fn test_memory() {
	assert!(assert_no_alloc(|| is_empty("")));
	assert!(assert_no_alloc(|| is_ascii("rust")));
	assert!(assert_no_alloc(|| contains("rust", "ru")));
	assert!(("ru", "st") == assert_no_alloc(|| split_at("rust", 2)));
	assert!(1 == assert_no_alloc(|| find("rust", 'u')));
}

#[test]
fn test_functions() {
	assert!(is_empty(""));
	assert!(!is_empty("something"));
	assert!(is_ascii("rust"));
	assert!(!is_ascii("rust¬"));
	assert!(contains("rust", "ru"));
	assert!(!contains("something", "mer"));
	assert_eq!(split_at("rust", 2), ("ru", "st"));
	assert_eq!(find("ru-st-e", '-'), 2);
	assert_eq!(find("ru-st-e", 'e'), 6);
}
