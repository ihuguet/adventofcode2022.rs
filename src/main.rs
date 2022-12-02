mod day01;
mod day02;

use std::env;

fn main() {
	let day = env::args().nth(1).unwrap()
				.parse::<u32>().unwrap();

	match day {
		1 => day01::run(),
		2 => day02::run(),
		_ => panic!()
	}
}
