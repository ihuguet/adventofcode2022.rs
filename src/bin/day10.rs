use adventofcode2022 as aoc;
use aoc::input::ParseAoCInputError;
use std::str::FromStr;

enum Instr {
	Noop,
	Add(i32),
}

fn main() {
	let program = aoc::input::parse_lines::<Instr>("day10");

	let mut x: i32 = 1;
	let mut cycles = 0;
	let mut signal_strength = 0;
	let cycles_check_strength = [20, 60, 100, 140, 180, 220];
	let mut screen = [[' '; 40]; 6];

	for instr in program {
		for _n in 0..instr.cycles() {
			// part 2
			let col = cycles % 40;
			let sprite = x - 1..=x + 1;
			if sprite.contains(&col) {
				screen[cycles as usize / 40][col as usize] = '#';
			}

			cycles += 1;

			// part 1
			if cycles_check_strength.contains(&cycles) {
				signal_strength += cycles * x;
			}
		}

		match instr {
			Instr::Add(v) => x += v,
			Instr::Noop => (),
		}
	}

	println!("Part 1: signal strength {}", signal_strength);
	println!("Part 2:");
	for row in screen {
		println!("{}", row.iter().collect::<String>());
	}
}

impl Instr {
	fn cycles(&self) -> usize {
		match self {
			Instr::Noop => 1,
			Instr::Add(_) => 2,
		}
	}
}

impl FromStr for Instr {
	type Err = aoc::input::ParseAoCInputError<Instr>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let words: Vec<&str> = s.split(" ").collect();
		match words[0] {
			"noop" => Ok(Instr::Noop),
			"addx" => Ok(Instr::Add(words[1].parse().unwrap())),
			_ => Err(ParseAoCInputError::new(s)),
		}
	}
}