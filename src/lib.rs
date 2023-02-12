use rand::Rng;
use std::cmp::{Eq, Ord, PartialOrd};
use std::fmt;

// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
// enum Face {
// Diamond,
// Heart,
// Spade,
// Club,
// }

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
	num: usize,
	// face: Face,
}

type Hand = Vec<Card>;

#[derive(Debug, PartialEq)]
pub struct Deck {
	pub cards: Hand,
}

impl Deck {
	pub fn new_shuffled() -> Self {
		let mut deck = Deck::default();
		deck.shuffle();
		deck
	}

	pub fn shuffle(&mut self) {
		let mut temp: Hand = Vec::new();
		let mut indexes = (0..self.len()).collect::<Vec<usize>>();
		let mut rng = rand::thread_rng();

		while !indexes.is_empty() {
			let index = rng.gen_range(0..indexes.len());
			let i = indexes[index];
			indexes.remove(index);
			temp.push(self.cards.get(i).unwrap().clone());
		}

		self.cards = temp;
	}

	pub fn pop(&mut self) -> Option<Card> {
		self.cards.pop()
	}

	pub fn push(&mut self, c: Card) {
		self.cards.push(c);
	}

	pub fn append(&mut self, c: Card) {
		let mut temp = self.cards.clone();
		self.cards = Vec::new();
		self.push(c);
		self.cards.append(&mut temp);
	}

	pub fn len(&self) -> usize {
		self.cards.len()
	}

	pub fn is_empty(&self) -> bool {
		self.len() < 1
	}
}

impl Default for Deck {
	fn default() -> Self {
		let mut deck = Deck { cards: Vec::new() };

		for n in 1..14 {
			for f in 1..5 {
				match f {
					1 => deck.push(Card {
						num: n,
						// face: Face::Diamond,
					}),
					2 => deck.push(Card {
						num: n,
						// face: Face::Heart,
					}),
					3 => deck.push(Card {
						num: n,
						// face: Face::Spade,
					}),
					4 => deck.push(Card {
						num: n,
						// face: Face::Club,
					}),
					_ => (),
				}
			}
		}

		deck
	}
}

impl fmt::Display for Deck {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut values = [0; 13];
		for card in &self.cards {
			values[card.num - 1] += 1;
		}
		for value in values {
			write!(f, "{},", value)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_card_comp() {
		let c1 = Card {
			num: 10,
			face: Face::Diamond,
		};
		let c2 = Card {
			num: 11,
			face: Face::Diamond,
		};

		assert!(c2 > c1);

		let c1 = Card {
			num: 10,
			face: Face::Diamond,
		};
		let c2 = Card {
			num: 10,
			face: Face::Heart,
		};

		assert!(c2 > c1);
	}

	fn test_pop_append() {
		let mut deck = Deck { cards: vec![] };

		deck.push(Card {
			num: 1,
			face: Face::Diamond,
		});
		deck.push(Card {
			num: 2,
			face: Face::Diamond,
		});

		let popped = deck.pop().unwrap();
		assert_eq!(
			&popped,
			&Card {
				num: 2,
				face: Face::Diamond,
			}
		);

		deck.append(popped);
		assert_eq!(
			deck,
			Deck {
				cards: vec![
					Card {
						num: 1,
						face: Face::Diamond,
					},
					Card {
						num: 2,
						face: Face::Diamond,
					}
				]
			}
		);
	}
}
