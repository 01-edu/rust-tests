/*
## string literal

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

pub fn is_empty(v: &str) -> bool {
	// v.split("").collect::<Vec<&str>>().len() == 0 // -> this is the bad one for example it will allocate on the heap
	v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
	v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
	v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
	v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
	v.find(pat).unwrap()
}
