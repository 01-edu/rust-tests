// Why does this code not compile

// It's not possible to have one mutable reference and one immutable

fn main() {
	let ref mut a = String::from("Hello");
	let len = a.len();
	let b = &a;

	add_excitement(a);

	println!("The len of {} is {}", a, len);
	println!("The length of {} is {}", b, b.len());
}

fn add_excitement(s: &mut String) {
	s.push_str("!");
}
