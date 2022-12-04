use adventofcode2022 as aoc;

fn main() {
	let lines = aoc::input::read_lines("day04");

	let mut count_part1 = 0;
	let mut count_part2 = 0;

	for line in lines {
		let mut days = line.split(&['-', ',']);
		let days1 = days.next().unwrap().parse::<u32>().unwrap()
					..=days.next().unwrap().parse::<u32>().unwrap();
		let days2 = days.next().unwrap().parse::<u32>().unwrap()
					..=days.next().unwrap().parse::<u32>().unwrap();

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