#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

impl Antigen {
    pub const EVERY: &[Self] = &[Self::A, Self::AB, Self::B, Self::O];
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

impl RhFactor {
    pub const EVERY: &[Self] = &[Self::Positive, Self::Negative];
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub rh_factor: RhFactor,
    pub antigen: Antigen,
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
