// Define a function that calculate the transpose matrix of a 2x2 matrix
// You don't need to understand everything about matrices

// Just convert lines into columns and vice versa
// ( a b )	__ transposition __> ( a d )
// ( c d )  					 ( b d )

// Only the body of the transpose function can be changed

// fn main() {
// 	let matrix = Matrix((1, 3), (4, 5));
// 	println!("Original matrix {:?}", matrix);
// 	println!("Transpose matrix {:?}", transpose(matrix));
// }

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
	Matrix(((m.0).0, (m.1).0), ((m.0).1, (m.1).1))
}
