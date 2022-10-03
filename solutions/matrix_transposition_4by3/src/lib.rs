// Define a function that calculate the transpose matrix of a 4x3 (4
// rows by 3 columns) matrix which is a 3x4 matrix (3 rows by 4 columns)

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
pub struct Matrix4by3(
    pub (i32, i32, i32),
    pub (i32, i32, i32),
    pub (i32, i32, i32),
    pub (i32, i32, i32),
);

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix3by4(
    pub (i32, i32, i32, i32),
    pub (i32, i32, i32, i32),
    pub (i32, i32, i32, i32),
);

pub fn transpose(m: Matrix4by3) -> Matrix3by4 {
    Matrix3by4(
        ((m.0).0, (m.1).0, (m.2).0, (m.3).0),
        ((m.0).1, (m.1).1, (m.2).1, (m.3).1),
        ((m.0).2, (m.1).2, (m.2).2, (m.3).2),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tranposion() {
        let matrix = Matrix4by3((1, 2, 3), (4, 5, 6), (7, 8, 9), (10, 11, 12));
        let expected = Matrix3by4((1, 4, 7, 10), (2, 5, 8, 11), (3, 6, 9, 12));
        assert_eq!(transpose(matrix), expected);
    }
}
