use poloto::build;

fn main() {
	// let dist = game::distribution(game::cats, 100000, 100);
	let dist = game::distribution(game::rhodes, 100000, 100);
	let mut dist: Vec<(f64, f64)> = dist
		.iter()
		.map(|(x, y)| (*x as f64, *y as f64))
		.collect();
	dist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

	let plots = poloto::plots!(build::plot("").line(dist), build::markers([], [0.0]));

	poloto::data(plots)
		.build_and_label((
			"Distribution of games per turn",
			"Turns",
			"# Games",
		))
		.append_to(poloto::header().dark_theme())
		.render_stdout();
}

mod game {
	use std::collections::HashMap;
	use std::ops::Range;
	use wars::*;

	pub fn distribution<F>(f: F, iter: usize, n_buckets: usize) -> HashMap<usize, usize>
	where
		F: Fn() -> usize,
	{
		let mut buckets = HashMap::<usize, usize>::new();
		let mut means = vec![0; iter];

		for i in 0..iter {
			means[i] = f();
		}

		let min = means.iter().min().unwrap();
		let max = means.iter().max().unwrap();
		let range: Range<usize> = Range {
			start: *min,
			end: *max,
		};
		let q = quanta(range, n_buckets);

		for mean in &means {
			for range in &q {
				if range.contains(mean) {
					let counter = buckets.entry(range.start).or_insert(0);
					*counter += 1;
				}
			}
		}

		buckets
	}

	pub fn quanta(range: Range<usize>, n: usize) -> Vec<Range<usize>> {
		let mut ranges = Vec::<Range<usize>>::new();
		let bucket_size = (range.end - range.start) / n;

		for i in 0..n {
			let bucket_start = (i * bucket_size) + range.start;
			ranges.push(bucket_start..bucket_start + bucket_size);
		}

		ranges
	}

	pub fn average<F>(f: F, iter: usize) -> usize
	where
		F: Fn() -> usize,
	{
		let mut sum: usize = 0;
		for _ in 0..iter {
			sum += f();
		}

		sum / iter
	}

	pub fn rhodes() -> usize {
		let deck = Deck::new_shuffled();
		let mut d1 = Deck {
			cards: deck.cards[0..deck.len() / 2].to_vec(),
		};
		let mut d2 = Deck {
			cards: deck.cards[deck.len() / 2..].to_vec(),
		};

		let mut count = 0;
		let mut pile = Vec::<Card>::new();
		while !d1.is_empty() && !d2.is_empty() {
			let c1 = d1.pop().unwrap();
			let c2 = d2.pop().unwrap();

			if c1 == c2 {
				(0..3).for_each(|_| {
					if let Some(c) = d1.pop() {
						pile.push(c);
					}
				});
				(0..3).for_each(|_| {
					if let Some(c) = d2.pop() {
						pile.push(c);
					}
				});
			} else if c1 > c2 {
				d1.push(c1);
				d1.push(c2);
				while !pile.is_empty() {
					d1.push(pile.pop().unwrap());
				}
			} else {
				d2.push(c1);
				d2.push(c2);
				while !pile.is_empty() {
					d2.push(pile.pop().unwrap());
				}
			}
			count += 1;
		}
		count
	}

	pub fn cats() -> usize {
		let deck = Deck::new_shuffled();
		let mut d1 = Deck {
			cards: deck.cards[0..deck.len() / 2].to_vec(),
		};
		let mut d2 = Deck {
			cards: deck.cards[deck.len() / 2..].to_vec(),
		};

		let mut count = 0;
		let mut pile = Vec::<Card>::new();
		while !d1.is_empty() && !d2.is_empty() {
			// print!("{}, ", count);
			let c1 = d1.pop().unwrap();
			let c2 = d2.pop().unwrap();

			if c1 == c2 {
				// print!("-, ");
				pile.push(c1);
				pile.push(c2);
			} else if c1 > c2 {
				// print!("{}, ", 1);
				d1.push(c1);
				d1.push(c2);
				while !pile.is_empty() {
					d1.push(pile.pop().unwrap());
				}
			} else {
				// print!("{}, ", 2);
				d2.push(c1);
				d2.push(c2);
				while !pile.is_empty() {
					d2.push(pile.pop().unwrap());
				}
			}

			// println!("{} ,{} ,{:?}", d1, d2, pile);
			count += 1;
		}
		count
	}
}
