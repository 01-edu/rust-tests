use std::ops::Deref;
//use std::mem::drop;

fn main() {
	let x = 5;
	let y = MyBox::new(x);

	assert_eq!(5, x);
	assert_eq!(5, *y);

	let a = 7;
	let c = &a;
	let b = Box::new(5);
	println!("b = {}", b);
	println!("c = {}", c);
	let name = String::from("Rust");
	hello(&name);
	let m = MyBox::new(String::from("Rust"));
	hello(&m);

	let c = CustomSmartPointer {
		data: String::from("my stuff"),
	};
	let d = CustomSmartPointer {
		data: String::from("other stuff"),
	};
	println!("CustomizeSmartPointers created c = {:?}, d = {:?}.", c, d);
	drop(c);
}

#[derive(Debug)]
struct CustomSmartPointer {
	data: String,
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data `{}`!", self.data);
	}
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

fn hello(name: &str) {
	println!("Hello, {}!", name);
}
