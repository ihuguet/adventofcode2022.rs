use adventofcode2022 as aoc;

pub fn main() {
	let lines = aoc::input::read_lines("day01");

	let mut elfs = Vec::new();
	let mut calories = 0;

	for line in lines {
		if line == "" {
			elfs.push(calories);
			calories = 0;
		} else {
			calories += line.parse::<u32>().unwrap();
		}
	}

	elfs.sort_by_key(|&x| std::cmp::Reverse(x));

	println!("Part 1: max calories {}", elfs[0]);
	println!("Part 2: top 3 calories {}", elfs[..3].iter().sum::<u32>())
}