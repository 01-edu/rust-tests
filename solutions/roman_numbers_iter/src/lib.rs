use std::iter;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla = 0,
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl RomanDigit {
    const fn base(value: u32) -> Self {
        use RomanDigit::*;

        if value >= M as u32 {
            M
        } else if value >= D as u32 {
            D
        } else if value >= C as u32 {
            C
        } else if value >= L as u32 {
            L
        } else if value >= X as u32 {
            X
        } else if value >= V as u32 {
            V
        } else if value >= I as u32 {
            I
        } else {
            Nulla
        }
    }

    fn take(value: &mut u32) -> Vec<Self> {
        let digits = value.checked_ilog10().unwrap_or_default() + 1;

        let d = *value / 10u32.pow(digits - 1);

        let ret = if (d + 1) % 5 == 0 {
            let v = Self::base(((d + 1) / 5) * 10u32.pow(digits - 1));
            *value -= d * 10u32.pow(digits - 1);
            vec![v, Self::base((d + 1) * 10u32.pow(digits - 1))]
        } else {
            let v = Self::base(*value);
            *value -= v as u32;
            vec![v]
        };

        ret
    }
}

impl From<u32> for RomanNumber {
    fn from(value: u32) -> Self {
        if value == 0 {
            return Self(vec![RomanDigit::Nulla]);
        }

        let mut acc = value;

        let it = iter::from_fn(|| {
            if acc == 0 {
                None
            } else {
                Some(RomanDigit::take(&mut acc))
            }
        })
        .flatten();

        Self(it.collect())
    }
}

impl RomanNumber {
    #[inline]
    fn to_u32(&self) -> u32 {
        u32::from(self)
    }
}

impl From<&RomanNumber> for u32 {
    #[inline]
    fn from(n: &RomanNumber) -> Self {
        n.0.iter()
            .copied()
            .map(|i| i as u32)
            .fold((0, 0), |(sum, prev), curr| {
                (sum + if prev < curr { curr - 2 * prev } else { curr }, curr)
            })
            .0
    }
}

impl Iterator for RomanNumber {
    type Item = Self;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(RomanNumber::from(self.to_u32() + 1))
        }
    }
}
