use adventofcode2022 as aoc;
use std::collections::{BTreeSet, BTreeMap};
use Dir::*;

type Point = aoc::space_2D::Point<isize>;
type Grid = BTreeSet<Point>;

#[derive(Clone, Copy)]
enum Dir {
	North, East, South, West
}

fn main() {
	let grid = parse_input("day23");
	println!("Part 1: empty spaces {}", part1(grid.clone()));
	println!("Part 2: {} rounds", part2(grid));
}

fn part1(mut grid: Grid) -> isize {
	let mut sides_order = [North, South, West, East];

	for _ in 0..10 {
		let tentative_moves = calc_tentative_moves(&grid, &sides_order);
		let moves = discard_collistions(tentative_moves);
		grid = move_elves(grid, moves);
		sides_order.rotate_left(1);
	}

	let (min, max) = get_minmax_points(&grid);
	(max.x - min.x + 1) * (max.y - min.y + 1) - grid.len() as isize
}

fn part2(mut grid: Grid) -> isize {
	let mut sides_order = [North, South, West, East];
	let mut round = 1;

	loop {
		let tentative_moves = calc_tentative_moves(&grid, &sides_order);
		let moves = discard_collistions(tentative_moves);
		if moves.len() == 0 {
			break;
		}
		grid = move_elves(grid, moves);
		sides_order.rotate_left(1);
		round += 1;
	}

	round
}

fn calc_tentative_moves(grid: &Grid, sides_order: &[Dir]) -> BTreeMap<Point, Vec<Point>> {
	let mut tentative_moves: BTreeMap<Point, Vec<Point>> = BTreeMap::new();

	for elf in grid {
		let counts: Vec<_> = sides_order.iter()
			.map(|side| (*side, count_elves(&grid, *elf, *side)))
			.collect();

		let count: usize = counts.iter()
			.map(|(_, count)| *count)
			.sum();

		let dir = counts.into_iter()
			.find(|(_, count)| *count == 0)
			.map(|(dir, _)| dir);

		if count > 0 && dir.is_some() {
			let next_pos = *elf + dir.unwrap().get_mov();
			tentative_moves.entry(next_pos)
				.or_default()
				.push(*elf);
		}
	}

	tentative_moves
}

fn count_elves(grid: &BTreeSet<Point>, elf: Point, side: Dir) -> usize {
	side.get_points().into_iter()
		.filter(|p| grid.contains(&(elf + *p)))
		.count()
}

fn discard_collistions(tentative_moves: BTreeMap<Point, Vec<Point>>) -> BTreeMap<Point, Point> {
	tentative_moves.into_iter()
		.filter(|(_dst, srcs)| srcs.len() == 1)
		.map(|(dst, srcs)| (srcs[0], dst))
		.collect()
}

fn move_elves(mut grid: Grid, moves: BTreeMap<Point, Point>) -> Grid {
	let mut grid_next = Grid::new();

	for (src, dst) in moves {
		grid_next.insert(dst);
		grid.remove(&src);
	}
	for elf in grid {
		grid_next.insert(elf);
	}

	grid_next
}

fn get_minmax_points(grid: &Grid) -> (Point, Point) {
	let mut min = (isize::MAX, isize::MAX);
	let mut max = (isize::MIN, isize::MIN);

	for elf in grid {
		if elf.y > max.0 {
			max.0 = elf.y;
		}
		if elf.y < min.0 {
			min.0 = elf.y;
		}
		if elf.x > max.1 {
			max.1 = elf.x;
		}
		if elf.x < min.1 {
			min.1 = elf.x;
		}
	}

	(min.into(), max.into())
}

impl Dir {
	fn get_points(&self) -> [Point; 3] {
		match self {
			North => [(-1, -1).into(), (-1, 0).into(), (-1, 1).into()],
			East  => [(1, 1).into(), (0, 1).into(), (-1, 1).into()],
			South => [(1, -1).into(), (1, 0).into(), (1, 1).into()],
			West  => [(1, -1).into(), (0, -1).into(), (-1, -1).into()]
		}
	}

	fn get_mov(&self) -> Point {
		match self {
			North => (-1, 0).into(),
			East  => (0, 1).into(),
			South => (1, 0).into(),
			West  => (0, -1).into()
		}
	}
}

fn parse_input(day_xx: &str) -> Grid {
	let mut grid = BTreeSet::new();

	for (y, line) in aoc::input::read_lines(day_xx).enumerate() {
		for (x, ch) in line.chars().enumerate() {
			if ch == '#' {
				grid.insert(Point::from((y as isize, x as isize)));
			}
		}
	}

	grid
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let grid = parse_input("day23-test");
		assert_eq!(part1(grid), 110);
	}
}
