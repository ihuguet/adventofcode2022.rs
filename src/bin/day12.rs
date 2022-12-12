use adventofcode2022 as aoc;
use aoc::space_2D::Grid as GridTrait;
use std::collections::BinaryHeap;

type Grid = aoc::space_2D::VecGrid<Terrain>;
type Point = aoc::space_2D::Point<usize>;

struct Terrain {
	height: u8,
	manhattan_dist: usize,
}

#[derive(PartialEq, Eq)]
struct State {
	cost: usize,
	steps: usize,
	point: Point
}

fn main() {
	let (grid, start, end) = parse_input();
	println!("Part 1: min steps {}", solve(&grid, start, end).unwrap());
	println!("Part 2: min steps {}", part2(&grid, end).unwrap());
}

fn solve(grid: &Grid, start: Point, end: Point) -> Option<usize> {
	let mut min_steps: Vec<Vec<usize>> = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
	let mut queue = BinaryHeap::new();
	queue.push(State::new(&grid, start, 0));

	while let Some(state) = queue.pop() {
		if state.point == end {
			continue;
		} else if min_steps[end] < state.cost {
			break;
		}

		let max_height_next = grid[state.point].height + 1;
		let steps_next = state.steps + 1;
		let points_next = grid.adjacents_4(state.point).into_iter()
			.filter(|&p| grid[p].height <= max_height_next)
			.collect::<Vec<_>>();

		for point_next in points_next {
			let prev_steps = &mut min_steps[point_next];
			if steps_next < *prev_steps {
				*prev_steps = steps_next;
				queue.push(State::new(&grid, point_next, steps_next));
			}
		}
	}

	match min_steps[end] {
		usize::MAX => None,
		min => Some(min)
	}
}

fn part2(grid: &Grid, end: Point) -> Option<usize> {
	grid.iter_grid()
		.filter_map(|(p, _)| to_start_candidate(grid, p))
		.filter_map(|start| solve(grid, start, end))
		.min()
}

fn to_start_candidate(grid: &Grid, point: Point) -> Option<Point> {
	let is_a_close_to_b = grid[point].height == b'a'
		&& grid.adjacents_4(point).into_iter().any(|p| grid[p].height == b'b');

	match is_a_close_to_b {
		true => Some(point),
		false => None
	}
}

fn parse_input() -> (Grid, Point, Point) {
	let mut start = Point::from((0, 0));
	let mut end = Point::from((0, 0));

	let mut grid = aoc::input::read_lines("day12").enumerate()
		.map(|(y, l)| {
			l.chars().enumerate().map(|(x, ch)| {
				let ch = match ch {
					'S' => { start = Point::from((y, x)); 'a' },
					'E' => { end = Point::from((y, x)); 'z' },
					ch => ch,
				};
				Terrain {height: ch as u8, manhattan_dist: 0}
			}).collect::<Vec<Terrain>>()
		}).collect::<Vec<Vec<Terrain>>>();

	for (point, terrain) in grid.iter_grid_mut() {
		terrain.manhattan_dist = end.y.abs_diff(point.y) + end.x.abs_diff(point.x);
	}

	(grid, start, end)
}

impl State {
	fn new(grid: &Grid, point: Point, steps: usize) -> Self {
		State { point, steps, cost: steps + grid[point].manhattan_dist }
	}
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		// comparation is inversed to make min-pri-queue
		other.cost.cmp(&self.cost).then_with(|| self.point.cmp(&other.point))
	}	
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}
