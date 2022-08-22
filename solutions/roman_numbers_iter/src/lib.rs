// ## roman_numbers_iter

// ### Instructions

// Implement the `Iterator` trait for the `RomanNumber` type. You should use the code from the previous exercise roman_numbers.

// ### Notions

// - [Trait Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

// ### Expected Functions

// ```rust
// //...

// impl Iterator for RomanNumber {}
// ```

// ### Usage

// Here is a program to test your function.

// ```rust
// use roman_numbers::RomanNumber;

// fn main() {
// 	let mut number = RomanNumber::from(15);

// 	println!("{:?}", number);
// 	println!("{:?}", number.next());
// }
// ```

// And its output

// ```console
// $ cargo run
// RomanNumber([X, V])
// Some(RomanNumber([X, V, I]))
// $
// ```

use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(n: u32) -> Self {
        match n {
            1..=4 => I,
            5..=9 => V,
            10..=49 => X,
            50..=99 => L,
            100..=499 => C,
            500..=999 => D,
            1000..=5000 => M,
            _ => Nulla,
        }
    }
}

impl From<RomanDigit> for u32 {
    fn from(n: RomanDigit) -> Self {
        match n {
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000,
            _ => 0,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        if n == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut quotient = n;
        let mut p = 0;
        let mut reverse_roman = Vec::new();

        while quotient != 0 {
            let rest = quotient % 10;
            quotient /= 10;
            p += 1;
            if rest == 9 {
                reverse_roman.push(RomanDigit::from(10_u32.pow(p)));
                reverse_roman.push(RomanDigit::from(10_u32.pow(p - 1)));
            } else if rest == 4 {
                reverse_roman.push(RomanDigit::from(10_u32.pow(p) / 2));
                reverse_roman.push(RomanDigit::from(10_u32.pow(p - 1)));
            } else if rest >= 5 {
                let repetitions = rest - 5;
                for _ in 0..repetitions {
                    reverse_roman.push(RomanDigit::from(10_u32.pow(p - 1)));
                }
                reverse_roman.push(RomanDigit::from(10_u32.pow(p) / 2));
            } else {
                for _ in 0..rest {
                    reverse_roman.push(RomanDigit::from(10_u32.pow(p - 1)))
                }
            }
        }

        reverse_roman.reverse();
        RomanNumber(reverse_roman)
    }
}

impl From<RomanNumber> for u32 {
    fn from(n: RomanNumber) -> Self {
        let mut result: u32 = 0;

        let numbers: Vec<u32> = n.0.iter().map(|i| u32::from(*i)).collect();

        for (i, value) in numbers.iter().enumerate() {
            result += value;
            if i >= 1 && *value > numbers[i - 1] {
                result = result - 2 * numbers[i - 1]
            }
        }

        result
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() == 0 {
            return None;
        }

        let mut aux = u32::from(self.clone());

        aux = aux + 1;

        Some(RomanNumber::from(aux))
    }
}
