use std::{fs, collections::BTreeSet};

fn main() {
	let signal = fs::read_to_string("input/day06.txt").unwrap();
	let signal = signal.trim().as_bytes();
	println!("Part 1: start-of-message at pos {}", find_unique_serie(&signal, 4).unwrap());
	println!("Part 2: start-of-packet at pos {}", find_unique_serie(&signal, 14).unwrap());
}

fn find_unique_serie(signal: &[u8], len: usize) -> Option<usize> {
	for (i, chars) in signal.windows(len).enumerate() {
		let chars: BTreeSet<u8> = chars.iter().copied().collect();
		if chars.len() == len {
			return Some(i + len);
		}
	}
	None
}
