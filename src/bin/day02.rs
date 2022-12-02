use adventofcode2022 as aoc;

pub fn main() {
	let lines = aoc::input::read_lines("day02");

	let points = lines.fold((0, 0), |totals, line| {
		let round = match line.as_str() {
			"A X" => (1 + 3, 3 + 0),
			"A Y" => (2 + 6, 1 + 3),
			"A Z" => (3 + 0, 2 + 6),
			"B X" => (1 + 0, 1 + 0),
			"B Y" => (2 + 3, 2 + 3),
			"B Z" => (3 + 6, 3 + 6),
			"C X" => (1 + 6, 2 + 0),
			"C Y" => (2 + 0, 3 + 3),
			"C Z" => (3 + 3, 1 + 6),
			_ => panic!(),
		};
		(totals.0 + round.0, totals.1 + round.1)
	});

	println!("Part 1: {} points", points.0);
	println!("Part 2: {} points", points.1);
}
