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
	let line_chunks = get_line_scanned_ranges(sensors, line_num);

	let chunks_sum = line_chunks.iter()
		.map(|range| (range.end() - range.start() + 1) as usize)
		.sum::<usize>();
	let line_beacons = sensors.iter()
		.filter_map(|sensor| if sensor.beacon.y == line_num {Some(sensor.beacon.x)} else {None})
		.collect::<BTreeSet<_>>();
	let beacons_sub = line_beacons.iter()
		.filter(|beacon| line_chunks.iter().any(|chunk| chunk.contains(beacon)))
		.count();

	chunks_sum - beacons_sub
}

fn part2(sensors: &[Sensor], search_size: isize) -> u64 {
	for line in 0..=search_size {
		let line_chunks = get_line_scanned_ranges(sensors, line);
		let unscanned_points = get_line_unscanned_points(&line_chunks, search_size);

		if !unscanned_points.is_empty() {
			return unscanned_points[0] as u64 * 4000000 + line as u64
		}
	}

	panic!("Expected 1 point, found 0");
}

fn get_line_scanned_ranges(sensors: &[Sensor], line_num: isize) -> Vec<RangeInclusive<isize>> {
	let mut chunks = Vec::new();

	for sensor in sensors {
		let beacon_rel = sensor.pos - sensor.beacon;
		let max_dist = beacon_rel.x.abs() + beacon_rel.y.abs();
		let x_diff = max_dist - (line_num - sensor.pos.y).abs();
		if x_diff >= 0 {
			chunks.push((sensor.pos.x - x_diff, sensor.pos.x + x_diff));
		}
	}

	chunks.sort();
	let chunks_merged = chunks.into_iter().fold(Vec::new(), chunk_append);
	chunks_merged
}

fn chunk_append(mut chunks: Vec<RangeInclusive<isize>>, chunk: (isize, isize)) -> Vec<RangeInclusive<isize>>{
	let chunk = chunk.0..=chunk.1;

	if let Some(prev) = chunks.pop() {
		if can_merge(&prev, &chunk) {
			let chunk = (*prev.start().min(chunk.start()), *prev.end().max(chunk.end()));
			return chunk_append(chunks, chunk);
		}
		chunks.push(prev);
	}

	chunks.push(chunk);
	chunks
}

fn can_merge(chunk1: &RangeInclusive<isize>, chunk2: &RangeInclusive<isize>) -> bool {
	chunk1.contains(chunk2.start()) || chunk1.contains(chunk2.end())
	|| chunk2.contains(chunk1.start()) || chunk2.contains(chunk1.end())
	|| chunk1.end() + 1 == *chunk2.start() || chunk2.start() + 1 == *chunk1.end()
}

fn get_line_unscanned_points(line_chunks: &Vec<RangeInclusive<isize>>, line_size: isize) -> Vec<isize> {
	let mut points = Vec::new();
	let scanned_start = *line_chunks[0].start();
	let scanned_end = *line_chunks.last().unwrap().end();

	for x in 0..scanned_start {
		points.push(x);
	}
	for x in scanned_end + 1..=line_size {
		points.push(x);
	}
	for chunks_pair in line_chunks.windows(2) {
		let unscanned_start = *chunks_pair[0].end() + 1;
		let unscanned_end = *chunks_pair[1].start();
		for x in unscanned_start..unscanned_end {
			points.push(x);
		}
	}

	points
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

		if let Some(cap) = RE.captures_iter(s).next() {
			Ok(Sensor {
				pos: Point::from((cap[2].parse()?, cap[1].parse()?)),
				beacon: Point::from((cap[4].parse()?, cap[3].parse()?))
			})
		} else {
			Err(ParseAoCInputError::new(s))
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