use adventofcode2022 as aoc;
use std::collections::BTreeSet;

type Point = (i32, i32 ,i32);

fn main() {
	let points = parse_input();
	println!("Part 1: {} exposed sides", part1(&points));
	println!("Part 2: {} exposed sides", part2(points));
}

fn part1(points: &BTreeSet<Point>) -> u32 {
	let dummy = BTreeSet::new();
	get_exposed_sides(&points, &dummy).len() as u32
}

fn part2(mut points: BTreeSet<Point>) -> u32 {
	let (min, max) = get_min_max(&points);

	let mut unchecked = BTreeSet::new();
	for x in min.0..=max.0 {
		for y in min.1..=max.1 {
			for z in min.2..=max.2 {
				if !points.contains(&(x, y, z)) {
					unchecked.insert((x, y, z));
				}
			}
		}
	}

	while !unchecked.is_empty() {
		let point = unchecked.iter().next().unwrap().clone();
		unchecked.remove(&point);

		let mut air_vol = BTreeSet::from([point]);

		loop {
			let exposed_sides = get_exposed_sides(&air_vol, &points);

			if exposed_sides.is_empty() {
				points.extend(air_vol.iter());
				unchecked.retain(|p| !air_vol.contains(&p));
				air_vol.clear();
				break;
			} else {
				air_vol.extend(exposed_sides.iter());
			}

			if out_of_bounds(&air_vol, min, max) {
				air_vol.clear();
				break;
			}
		}
	}

	part1(&points)
}

fn get_exposed_sides(vol_to_check: &BTreeSet<Point>, other_vol: &BTreeSet<Point>) -> Vec<Point> {
	vol_to_check.into_iter()
		.flat_map(|point| {
			adjacents(*point).filter(|p| !vol_to_check.contains(p) && !other_vol.contains(p))
		})
		.collect()
}

fn adjacents(point: Point) -> impl Iterator<Item = Point> {
	let adjs = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
	adjs.into_iter().map(move |adj| (point.0 + adj.0, point.1 + adj.1, point.2 + adj.2))
}

fn out_of_bounds(vol: &BTreeSet<Point>, min: Point, max: Point) -> bool {
	let (vol_min, vol_max) = get_min_max(vol);
	vol_min.0 < min.0 || vol_min.1 < min.1 || vol_min.2 < min.2
		|| vol_max.0 > max.0 || vol_max.1 > max.1 || vol_max.2 > max.2
}

fn get_min_max(points: &BTreeSet<Point>) -> (Point, Point) {
	let mut min = (i32::MAX, i32::MAX, i32::MAX);
	let mut max = (0, 0, 0);

	for point in points {
		match point.0 {
			v if v < min.0 => min.0 = v,
			v if v > max.0 => max.0 = v,
			_ => ()
		}
		match point.1 {
			v if v < min.1 => min.1 = v,
			v if v > max.1 => max.1 = v,
			_ => ()
		}
		match point.2 {
			v if v < min.2 => min.2 = v,
			v if v > max.2 => max.2 = v,
			_ => ()
		}
	}

	(min, max)
}

fn parse_input() -> BTreeSet<Point> {
	aoc::input::read_tokens_split_str::<i32>("day18", ",")
		.map(|tokens| (tokens[0], tokens[1], tokens[2]))
		.collect()
}