use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
	let f = File::open("input/day01.txt").unwrap();
	let reader = BufReader::new(f);

	let mut elfs = Vec::new();
	let mut calories = 0;

	for line in reader.lines().map(|l| l.unwrap()) {
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