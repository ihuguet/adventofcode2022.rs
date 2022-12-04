use adventofcode2022 as aoc;

fn main() {
	let lines = aoc::input::read_tokens_split_chars::<u32>("day04", &['-', ',']);

	let mut count_part1 = 0;
	let mut count_part2 = 0;

	for line in lines {
		let days1 = line[0]..=line[1];
		let days2 = line[2]..=line[3];

		if (days2.contains(&days1.start()) && days2.contains(&days1.end()))
				|| (days1.contains(&days2.start()) && days1.contains(&days2.end())) {
			count_part1 += 1
		}

		if days2.contains(&days1.start()) || days2.contains(&days1.end())
				|| days1.contains(&days2.start()) || days1.contains(&days2.end()) {
			count_part2 += 1;
		}
	}

	println!("Part 1: {} fully overlapped areas", count_part1);
	println!("Part 2: {} total overlapped areas", count_part2);
}