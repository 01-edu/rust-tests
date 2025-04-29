use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Rank {
    #[inline]
    pub fn random() -> Self {
        Self::translate(rand::thread_rng().gen_range(1..14))
    }

    #[inline]
    pub fn translate(value: u8) -> Self {
        match value {
            1 => Self::Ace,
            n @ 2..=10 => Self::Number(n),
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => unreachable!(),
        }
    }
}

impl Suit {
    #[inline]
    pub fn random() -> Self {
        Self::translate(rand::thread_rng().gen_range(1..5))
    }

    #[inline]
    pub fn translate(value: u8) -> Self {
        match value {
            1 => Self::Heart,
            2 => Self::Diamond,
            3 => Self::Spade,
            4 => Self::Club,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[inline]
pub fn winner_card(card: &Card) -> bool {
    card == &Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    }
}
