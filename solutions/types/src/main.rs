// Using your knowledge of generics
// create a structure called `Types` and a method `new` with the ability
// to store any type of values

#[derive(Debug)]
struct Types<T> {
	value: T,
}

impl<T> Types<T> {
	fn new(value: T) -> Self {
		Types { value }
	}
}

fn main() {
	let a = Types::new("hello");
	let b = Types::new(21);
	let c = Types::new(vec![1, 2, 3]);

	println!("{:?}", a);
	println!("{:?}", b);
	println!("{:?}", c);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn wrapper_u32() {
		assert_eq!(Types::new(42).value, 42);
	}

	#[test]
	fn wrapper_str() {
		assert_eq!(Types::new("Foo").value, "Foo");
	}

	#[test]
	fn wrapper_f64() {
		assert_eq!(Types::new(1.00).value, 1.00);
	}

	#[test]
	fn wrapper_vec() {
		assert_eq!(Types::new(vec![1, 2, 3]).value, [1, 2, 3]);
	}
}
