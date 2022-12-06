use std::fs;
use std::collections::BTreeSet;

fn main() {
	let signal = fs::read_to_string("input/day06.txt").unwrap();
	let signal = signal.trim().as_bytes();

	for i in 0..signal.len() {
		let chars: BTreeSet<u8> = signal[i..i+4].iter().copied().collect();
		if chars.len() == 4 {
			println!("Part 1: start-of-message at pos {}", i + 4);
			break;
		}
	}

	for i in 0..signal.len() {
		let chars: BTreeSet<u8> = signal[i..i+14].iter().copied().collect();
		if chars.len() == 14 {
			println!("Part 2: start-of-packet at pos {}", i + 14);
			break;
		}
	}
}