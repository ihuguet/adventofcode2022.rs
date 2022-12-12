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
	let mut queue = BinaryHeap::new();
	queue.push(State::new(&grid, start, 0));
	let mut min_steps: Vec<Vec<Option<usize>>> = vec![vec![None; grid[0].len()]; grid.len()];
	let mut min_steps_total = None;

	while let Some(state) = queue.pop() {
		if state.point == end {
			if min_steps_total.is_none() || state.steps < min_steps_total.unwrap() {
				min_steps_total.replace(state.steps);
			}
			continue;
		} else if min_steps_total.is_some() && min_steps_total.unwrap() <= state.cost {
			break;
		}

		let max_height_next = grid[state.point].height + 1;
		let steps_next = state.steps + 1;

		let points_next = grid.adjacents_4(state.point).into_iter()
			.filter(|&p| grid[p].height <= max_height_next)
			.collect::<Vec<_>>();

		for &point_next in points_next.iter() {
			let prev_steps = &mut min_steps[point_next];
			if prev_steps.is_none() || steps_next < prev_steps.unwrap() {
				prev_steps.replace(steps_next);
				queue.push(State::new(&grid, point_next, steps_next));
			}
		}
	}

	min_steps_total
}

fn part2(grid: &Grid, end: Point) -> Option<usize> {
	// only 'a' terrains next to a 'b' terrain are good candidates
	let start_candidates = grid.iter_grid().filter(|(p, _)| {
		grid[*p].height == b'a' && grid.adjacents_4(*p).into_iter().any(|p2| grid[p2].height == b'b')
	}).map(|(p, _)| p);

	start_candidates.filter_map(|start| solve(grid, start, end)).min()
}

fn parse_input() -> (Grid, Point, Point) {
	let mut input = aoc::input::read_lines("day12")
		.map(|l| l.chars().map(|ch| Terrain {height: ch as u8, manhattan_dist: 0}).collect())
		.collect::<Vec<_>>();
	let mut start: Option<Point> = None;
	let mut end: Option<Point> = None;

	for (point, terrain) in input.iter_grid_mut() {
		match terrain.height {
			b'S' => { start = Some(point); terrain.height = b'a'; },
			b'E' => { end = Some(point); terrain.height = b'z'; },
			_ => () 
		}
		if start.is_some() && end.is_some() {
			break;
		}
	}

	let _end = end.unwrap();
	for (point, terrain) in input.iter_grid_mut() {
		terrain.manhattan_dist = _end.y.abs_diff(point.y) + _end.x.abs_diff(point.x);
	}

	(input, start.unwrap(), end.unwrap())
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
