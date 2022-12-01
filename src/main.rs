mod day01;

use std::env;

fn main() {
	let day = env::args().nth(1).unwrap()
				.parse::<u32>().unwrap();

	match day {
		1 => day01::run(),
		_ => panic!()
	}
}
