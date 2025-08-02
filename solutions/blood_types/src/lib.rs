use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub rh_factor: RhFactor,
    pub antigen: Antigen,
}

impl Antigen {
    pub const EVERY: &[Self] = &[Self::A, Self::AB, Self::B, Self::O];
}

impl fmt::Debug for Antigen {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::A => "A",
                Self::AB => "AB",
                Self::B => "B",
                Self::O => "O",
            }
        )
    }
}

impl FromStr for Antigen {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "AB" => Ok(Self::AB),
            "B" => Ok(Self::B),
            "O" => Ok(Self::O),
            _ => Err(()),
        }
    }
}

impl RhFactor {
    pub const EVERY: &[Self] = &[Self::Positive, Self::Negative];
}

impl fmt::Debug for RhFactor {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Positive => "+",
                Self::Negative => "-",
            }
        )
    }
}

impl FromStr for RhFactor {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Positive),
            "-" => Ok(Self::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen, rh) = s.split_at_checked(s.len() - 1).ok_or(())?;

        Ok(Self {
            antigen: antigen.parse()?,
            rh_factor: rh.parse()?,
        })
    }
}

impl fmt::Debug for BloodType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.antigen, self.rh_factor)
    }
}

impl BloodType {
    #[inline]
    pub fn every() -> impl Iterator<Item = Self> {
        Antigen::EVERY.iter().copied().flat_map(|a| {
            RhFactor::EVERY.iter().copied().map(move |r| Self {
                antigen: a,
                rh_factor: r,
            })
        })
    }

    #[inline]
    fn can_donate_to(self, other: Self) -> bool {
        other.can_receive_from(self)
    }

    pub fn can_receive_from(self, other: Self) -> bool {
        if !(self.rh_factor == other.rh_factor || self.rh_factor == RhFactor::Positive) {
            return false;
        }

        other.antigen == Antigen::O || self.antigen == Antigen::AB || other.antigen == self.antigen
    }

    #[inline]
    pub fn donors(self) -> Vec<Self> {
        Self::every()
            .filter(|&b| self.can_receive_from(b))
            .collect()
    }

    #[inline]
    pub fn recipients(self) -> Vec<Self> {
        Self::every().filter(|&b| self.can_donate_to(b)).collect()
    }
}
