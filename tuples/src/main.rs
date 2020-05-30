// Define a tuple to represent a student
// Each student is identified by an id number of type i32
// Its name and its last name as a string
// Print the content of the tuple to stdout

// Note: &str is a string slice in Rust for this exercise you just need
// to know that string literals like "hello" are string slices
fn main() {
	let student: (i32, &str, &str) = (20, "Marie", "Malarme");
	println!("Student: {:?}", student);
	// After print only the first name
	println!("Student first name: {}", student.1);
	// After print only the last name
	println!("Student last name: {}", student.2);
	// After print only the id
	println!("Student Id: {}", student.0);
}
