// Define a tuple to represent a student
// Each student is identified by an id number of type i32
// Its name and its last name as a string
// Print the content of the tuple to stdout

#[derive(Debug)]
struct Student(i32, String, String);

fn main() {
	let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
	println!("Student: {:?}", student);
	// After print only the first name
	println!("Student first name: {}", student.1);
	// After print only the last name
	println!("Student last name: {}", student.2);
	// After print only the id
	println!("Student Id: {}", student.0);
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn it_works() {
		let student = Student(20, String::from("Pedro"), String::from("Domingos"));
		assert_eq!(student.0, 20);
		assert_eq!(student.1, "Pedro".to_string());
		assert_eq!(student.2, "Domingos".to_string());
	}
}
