use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Rank {
    pub fn random() -> Rank {
        let value: u8 = rand::thread_rng().gen_range(1..14);
        Rank::translate(value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            n @ 2..=10 => Rank::Number(n),
            11 => Rank::Jack,
            12 => Rank::Queen,
            _ => Rank::King,
        }
    }
}

impl Suit {
    pub fn random() -> Suit {
        let value = rand::thread_rng().gen_range(1..5);
        Suit::translate(value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    Card {
        suit: Suit::Spade,
        rank: Rank::Ace,
    } == *card
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winner() {
        let winner = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };
        for rank in 1..14 {
            for suit in 1..5 {
                let card = Card {
                    rank: Rank::translate(rank),
                    suit: Suit::translate(suit),
                };
                if card != winner {
                    assert!(!winner_card(&card));
                } else {
                    assert!(winner_card(&card));
                }
            }
        }
    }
}
