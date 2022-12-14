use adventofcode2022 as aoc;
use aoc::space_2D::{Point, VecGrid, Grid};
use Material::*;

const SAND_START_POS: Point = Point {y: 0, x: 500};

#[derive(Debug, Clone, PartialEq, Copy)]
enum Material {
	Air,
	Rock,
	Sand
}

fn main() {
	let grid = parse_input();
	println!("Part 1: {} units of sand", part1(grid.clone()));
	println!("Part 2: {} units of sand", part2(grid));
}

fn part1(mut grid: VecGrid<Material>) -> u32 {
	let max_rock_y = calc_max_rock_y(&grid).unwrap();
	let mut sand_count = 0;

	loop {
		let mut pos = SAND_START_POS;
		while let Some(next) = next_pos(&grid, pos, None) {
			pos = next;
			if pos.y > max_rock_y {
				return sand_count;
			}
		}
		grid[pos] = Sand;
		sand_count += 1;
	}
}

fn part2(mut grid: VecGrid<Material>) -> u32 {
	let floor_y = calc_max_rock_y(&grid).unwrap() + 2;
	let mut sand_count = 0;

	while grid[SAND_START_POS] != Sand {
		let mut pos = SAND_START_POS;
		while let Some(next) = next_pos(&grid, pos, Some(floor_y)) {
			pos = next;
		}
		grid[pos] = Sand;
		sand_count += 1;
	}

	sand_count
}

fn next_pos(grid: &VecGrid<Material>, pos: Point, max_y: Option<usize>) -> Option<Point> {
	if max_y.is_some() && pos.y + 1 == max_y.unwrap() {
		return None;
	}

	let left = pos.add_signed((1, -1).into());
	let center = pos.add_signed((1, 0).into());
	let right = pos.add_signed((1, 1).into());

	match (grid[center], grid[left], grid[right]) {
		(Air, _l, _r) => Some(center),
		(_c, Air, _r) => Some(left),
		(_c, _l, Air) => Some(right),
		(_c, _l, _r)  => None,
	}
}

fn calc_max_rock_y(grid: &VecGrid<Material>) -> Option<usize> {
	grid.iter_grid().filter_map(|(p, m)| if *m == Rock {Some(p.y)} else {None}).max()
}

fn parse_input() -> VecGrid<Material> {
	let input = aoc::input::read_tokens_split_str::<String>("day14", " -> ")
		.map(|coords| coords.into_iter().map(|coord| {
				let coord = coord.split(",").collect::<Vec<_>>();
				let x = coord[0].parse().unwrap();
				let y = coord[1].parse().unwrap();
				Point::from((y, x))
			}).collect::<Vec<Point>>()
		).collect::<Vec<Vec<Point>>>();
	
	let max_y = input.iter().flat_map(|l| l.iter().map(|p| p.y)).max().unwrap();
	let max_x = input.iter().flat_map(|l| l.iter().map(|p| p.x)).max().unwrap();
	let mut grid = vec![vec![Air; max_x * 2]; max_y + 2];

	// create the lines of rock joining all the line's points
	for mut line in input {
		let mut point = line.pop().unwrap();
		grid[point] = Rock;

		while let Some(next) = line.pop() {
			let sum: Point<isize> = (next - point).signum().into();
			while point != next {
				point = point.add_signed(sum);
				grid[point] = Rock;
			}
		}
	}

	grid
}