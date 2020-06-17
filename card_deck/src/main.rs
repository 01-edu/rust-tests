// Create a enum that represent the card suits
use rand::Rng;

#[derive(Debug)]
enum Suit {
	Heart,
	Diamond,
	Spade,
	Club,
}

#[derive(Debug)]
enum Rank {
	Ace,
	King,
	Queen,
	Jack,
	Number(u8),
}

impl Rank {
	fn random() -> Rank {
		let value: u8 = rand::thread_rng().gen_range(1, 14);
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
	fn random() -> Suit {
		let value = rand::thread_rng().gen_range(1, 5);
		match value {
			1 => Suit::Heart,
			2 => Suit::Diamond,
			3 => Suit::Spade,
			_ => Suit::Club,
		}
	}
}

#[derive(Debug)]
struct Card {
	suit: Suit,
	rank: Rank,
}

// Write a program that takes that returns a random card in the deck
// A standard deck of cards has 52 cards: 4 suits and 13 cards per suit
fn main() {
	let your_card = Card {
		rank: Rank::random(),
		suit: Suit::random(),
	};

	println!("You're card is a {:?}", your_card);

	// Now if the card is an Ace of Spades print "You are the winner"
	if let Card {
		suit: Suit::Spade,
		rank: Rank::Ace,
	} = your_card
	{
		println!("You are the winner!");
	}
}
