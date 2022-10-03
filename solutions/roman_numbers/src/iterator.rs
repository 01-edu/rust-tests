// # Instructions
// Implement the IntoIterator trait for the `RomanNumber` type to
// enable using a for loop notation.
// This implementation must allow taking ownership,
// borrowing and borrowing mutably

// I.e. this three constructions must be possible
// ```rust
// let number = RomanNumber::from(23);

// 1. Taking ownership (this consumes the RomanNumber)
// for digit in number {
// 	...
// }

// 2. Borrowing immutably (this preserves the RomanNumber)
// 	for digit in &number {

// 	}

// 3. Borrowing mutably (this allow you to modify the RomanNumber
// without having to return the ownership)
// 	for digit in &mut number {

// 	}

// Start with your research See https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
// https://doc.rust-lang.org/std/iter/index.html

use crate::{RomanDigit, RomanNumber};

impl<'a> IntoIterator for &'a RomanNumber {
    type Item = &'a RomanDigit;
    type IntoIter = std::slice::Iter<'a, RomanDigit>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut RomanNumber {
    type Item = &'a mut RomanDigit;
    type IntoIter = std::slice::IterMut<'a, RomanDigit>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl IntoIterator for RomanNumber {
    type Item = RomanDigit;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn into_u32(n: RomanDigit) -> u32 {
        use RomanDigit::*;
        match n {
            Nulla => 0,
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000,
        }
    }

    #[test]
    fn test_iter() {
        let number = RomanNumber::from(15);

        for digit in &number {
            println!("{:?}", digit);
        }
        println!("{:?}", number);
    }

    #[test]
    fn test_into_iter() {
        let number = RomanNumber::from(37);
        let value: u32 = number.into_iter().map(|digit| into_u32(digit)).sum();
        println!("value: {}", value);
    }

    #[test]
    fn test_iter_mut() {
        let mut number = RomanNumber::from(22);

        for digit in &mut number {
            let value = into_u32(*digit);
            *digit = dbg!(RomanNumber::from(value - 1)).0[0];
        }
        println!(
            "Roman Number after increasing the each digit by 1 = {:?}",
            number
        );
    }
}
