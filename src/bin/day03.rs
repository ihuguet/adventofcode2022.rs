use adventofcode2022 as aoc;
use std::collections::BTreeSet;

fn main() {
	let lines = aoc::input::read_lines("day03").collect();
	part1(&lines);
	part2(&lines);
}

fn part1(lines: &Vec<String>) {
	let mut points = 0;

	for line in lines {
		let half1 = &line[..line.len() / 2];
		let half2 = &line[line.len() / 2..];

		let found_chs : BTreeSet<char> =
			half1.chars()
				.filter(|&ch| half2.find(ch).is_some())
				.collect();
		
		for ch in found_chs {
			points += calc_points(ch);
		}
	}

	println!("Part 1: {} points", points);
}

fn part2(lines: &Vec<String>) {
	let mut points = 0;

	for i in (0..lines.len()).step_by(3) {
		for ch in lines[i].chars() {
			if lines[i + 1].find(ch).is_some() && lines[i + 2].find(ch).is_some() {
				points += calc_points(ch);
				break;
			}
		}
	}

	println!("Part 2: {} points", points);
}

fn calc_points(ch: char) -> u32 {
	let ch = ch as u32;
	if ch >= 'a' as u32 {
		ch - 'a' as u32 + 1
	} else {
		ch - 'A' as u32 + 27
	}
}