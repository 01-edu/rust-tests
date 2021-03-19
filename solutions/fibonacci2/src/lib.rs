// fn main() {
// 	let val: u32 = 20;
// 	println!("Fibonacci({}) = {}", val, fibonacci(val));
//}

pub fn fibonacci(n: u32) -> u32 {
	if n < 2 {
		return n;
	}
	fibonacci(n - 2) + fibonacci(n - 1)
}

fn main() {
	println!("fibonacci(3) = {}", fibonacci(3));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(fibonacci(0), 0);
		assert_eq!(fibonacci(1), 1);
		assert_eq!(fibonacci(22), 17711);
		assert_eq!(fibonacci(20), 6765);
	}
}
