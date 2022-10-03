// # Instructions
// Implement the From<u32> Trait to create a roman number from a u32
// the roman number should be in subtractive notation (the common way to write roman
// number I, II, II, IV, V, VI, VII, VIII, IX, X ...)

// For this start by defining the digits as `RomanDigit` with the values
// I, V, X, L, C, D, M and Nulla for 0

// Next define RomanNumber as a wrapper to a vector of RomanDigit's
// And implement the Trait From<u32>

// Examples:
// RomanNumber::from(32) = [X,X,X,I,I]
// RomanNumber::from(9) = [I,X]
// RomanNumber::from(45) = [X,L,V]
// RomanNumber:;from(0) = [Nulla]

mod iterator;
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

#[derive(Debug)]
pub struct RomanNumber(pub Vec<RomanDigit>);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(RomanNumber::from(3).0, [I, I, I]);
        assert_eq!(RomanNumber::from(6).0, [V, I]);
        assert_eq!(RomanNumber::from(15).0, [X, V]);
        assert_eq!(RomanNumber::from(30).0, [X, X, X]);
        assert_eq!(RomanNumber::from(150).0, [C, L]);
        assert_eq!(RomanNumber::from(200).0, [C, C]);
        assert_eq!(RomanNumber::from(600).0, [D, C]);
        assert_eq!(RomanNumber::from(1500).0, [M, D]);
    }

    #[test]
    fn substractive_notation() {
        assert_eq!(RomanNumber::from(4).0, [I, V]);
        assert_eq!(RomanNumber::from(44).0, [X, L, I, V]);
        assert_eq!(RomanNumber::from(3446).0, [M, M, M, C, D, X, L, V, I]);
        assert_eq!(RomanNumber::from(9).0, [I, X]);
        assert_eq!(RomanNumber::from(94).0, [X, C, I, V]);
    }
}
