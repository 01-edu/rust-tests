// Why does this code not compile?

// a) `ref` cannot be used for compound data types
// b) Strings are always immutable
// c) There cannot be more than one immutable reference to the same variable
// d) A reference cannot be printed by `println!`

fn main() {
	let mut s = String::from("hello");
	let r1 = &mut s;
	let ref mut r2 = s;

	println!("{}, {}", r1, r2);
}
