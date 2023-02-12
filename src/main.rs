use wars::*;

fn main() {
	let deck = Deck::new_shuffled();
	let mut d1 = Deck {
		cards: deck.cards[0..deck.len()/2].to_vec()
	};
	let mut d2 = Deck {
		cards: deck.cards[deck.len()/2..].to_vec()
	};

	let mut count = 0;
	let mut pile = Vec::<Card>::new();
	while !d1.is_empty() && !d2.is_empty() {
		print!("{}, ", count);
		let c1 = d1.pop().unwrap();
		let c2 = d2.pop().unwrap();

		if c1 == c2 {
			print!("-, ");
			pile.push(c1);
			pile.push(c2);
		} else if c1 > c2 {
			print!("{}, ", 1);
			d1.push(c1);
			d1.push(c2);
			while !pile.is_empty() {
				d1.push(pile.pop().unwrap());
			}
		} else {
			print!("{}, ", 2);
			d2.push(c1);
			d2.push(c2);
			while !pile.is_empty() {
				d2.push(pile.pop().unwrap());
			}
		}

		println!("{} ,{} ,{:?}", d1, d2, pile);
		count += 1;
	}
}
