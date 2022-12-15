use adventofcode2022 as aoc;
use aoc::input::ParseAoCInputError;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::collections::BTreeSet;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

const PART1_LINE: isize = 2_000_000;
const PART2_SEARCH_SIZE: isize = 4_000_000;

type Point = aoc::space_2D::Point<isize>;

#[derive(Debug)]
struct Sensor {
	pos: Point,
	beacon: Point,
}

fn main() {
	let sensors = parse_input("day15");
	println!("Part 1: {} points without beacon", part1(&sensors, PART1_LINE));
	println!("Part 2: frequency {}", part2(&sensors, PART2_SEARCH_SIZE));
}

fn part1(sensors: &[Sensor], line_num: isize) -> usize {
	let scanned_ranges = get_line_scanned_ranges(sensors, line_num);

	let scanned_points_count = scanned_ranges.into_iter()
		.fold(Vec::new(), range_append_merging).into_iter()
		.map(|range| (range.end() - range.start() + 1) as usize)
		.sum::<usize>();
	let line_beacons = sensors.iter()
		.filter(|sensor| sensor.beacon.y == line_num)
		.map(|sensor| sensor.beacon.x)
		.collect::<BTreeSet<_>>();

	scanned_points_count - line_beacons.len()
}

fn part2(sensors: &[Sensor], search_size: isize) -> u64 {
	for line in 0..=search_size {
		let scanned_ranges = get_line_scanned_ranges(sensors, line);

		if let Some(pos) = find_line_unscanned_point(&scanned_ranges, search_size) {
			return pos as u64 * 4000000 + line as u64
		}
	}

	panic!("Expected 1 point, found 0");
}

fn get_line_scanned_ranges(sensors: &[Sensor], line_num: isize) -> Vec<(isize, isize)> {
	let mut ranges = Vec::new();

	for sensor in sensors {
		let beacon_rel = sensor.pos - sensor.beacon;
		let max_dist = beacon_rel.x.abs() + beacon_rel.y.abs();
		let x_diff = max_dist - (line_num - sensor.pos.y).abs();
		if x_diff >= 0 {
			ranges.push((sensor.pos.x - x_diff, sensor.pos.x + x_diff));
		}
	}

	ranges.sort();
	ranges
}

fn range_append_merging(mut ranges_merged: Vec<RangeInclusive<isize>>, range: (isize, isize))
	-> Vec<RangeInclusive<isize>>
{
	let range = range.0..=range.1;

	if let Some(prev) = ranges_merged.pop() {
		if can_merge(&prev, &range) {
			let range = (*prev.start().min(range.start()), *prev.end().max(range.end()));
			return range_append_merging(ranges_merged, range);
		}
		ranges_merged.push(prev);
	}

	ranges_merged.push(range);
	ranges_merged
}

fn can_merge(range1: &RangeInclusive<isize>, range2: &RangeInclusive<isize>) -> bool {
	range1.contains(range2.start()) || range1.contains(range2.end())
	|| range2.contains(range1.start()) || range2.contains(range1.end())
	|| range1.end() + 1 == *range2.start() || range2.start() + 1 == *range1.end()
}

fn find_line_unscanned_point(scanned_ranges: &Vec<(isize, isize)>, line_size: isize)
	-> Option<isize>
{
	let mut pos = 0;
	for range in scanned_ranges {
		if pos >= range.0 && pos <= range.1 {
			pos = range.1 + 1;
		}
	}

	match pos {
		pos if pos <= line_size => Some(pos),
		_ => None
	}
}

fn parse_input(day_xx: &str) -> Vec<Sensor> {
	aoc::input::parse_lines(day_xx).collect()
}

impl FromStr for Sensor {
	type Err = ParseAoCInputError<Self>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
		}

		match RE.captures_iter(s).next() {
			Some(cap) => Ok(Sensor {
				pos: Point::from((cap[2].parse()?, cap[1].parse()?)),
				beacon: Point::from((cap[4].parse()?, cap[3].parse()?))
			}),
			None => Err(ParseAoCInputError::new(s))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let sensors = parse_input("day15-test");
		assert_eq!(part1(&sensors, 10), 26);
	}

	#[test]
	fn test_part2() {
		let sensors = parse_input("day15-test");
		assert_eq!(part2(&sensors, 20), 56000011);
	}
}