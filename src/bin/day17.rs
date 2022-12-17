use adventofcode2022 as aoc;
use aoc::space_2D::{Point, Grid, VecGrid};
use std::collections::HashMap;

const TOWER_WIDTH: usize = 7;
const RIGHT: Point<isize> = Point {y: 0, x: 1};
const LEFT: Point<isize> = Point {y: 0, x: -1};
const DOWN: Point<isize> = Point {y: -1, x: 0};

#[derive(PartialEq)]
struct Shape(Vec<Vec<bool>>);

type State = (usize, usize, Vec<bool>);

struct Tower {
	grid: VecGrid<bool>,
	height: usize,
	height_sum: usize,
	rocks_count: usize,
	states: HashMap<State, (usize, usize)>
}

fn main() {
	let jet_pattern = parse_input();
	let shapes = create_shapes();
	println!("Part 1: tower {} high", Tower::solve(&jet_pattern, &shapes, 2022));
	println!("Part 2: tower {} high", Tower::solve(&jet_pattern, &shapes, 1000000000000));
}

impl Tower {
	fn new() -> Self {
		Tower {
			grid: VecGrid::new(),
			height: 0,
			height_sum: 0,
			rocks_count: 0,
			states: HashMap::new()
		}
	}

	fn solve(jet_pattern: &[Point<isize>], shapes: &[Shape], limit: usize) -> usize {
		let mut tower = Tower::new();
		let mut jets_iter = jet_pattern.iter().copied().enumerate().cycle();
		let mut shapes_iter = shapes.into_iter().enumerate().cycle();

		let (mut shape_n, mut shape) = shapes_iter.next().unwrap();
		let mut pos = tower.prepare_next_rock_and_get_pos();

		while tower.rocks_count < limit {
			// left/right
			let (mov_n, mov) = jets_iter.next().unwrap();
			if let Ok(mov) = tower.check_move(shape, pos, mov) {
				pos = pos.add_signed(mov);
			}

			// down
			if let Ok(mov) = tower.check_move(shape, pos, DOWN) {
				pos = pos.add_signed(mov);
			} else {
				tower.place_shape(shape, pos);
				tower.maybe_fast_advance(limit, shape_n, mov_n);

				(shape_n, shape) = shapes_iter.next().unwrap();
				pos = tower.prepare_next_rock_and_get_pos();
			}
		}

		tower.height + tower.height_sum
	}

	fn prepare_next_rock_and_get_pos(&mut self) -> Point {
		let pos = Point::from((self.height + 3, 2));

		while self.grid.len() < pos.y + 4 {
			self.grid.push(vec![false; TOWER_WIDTH]);
		}

		pos
	}

	fn check_move(&self, shape: &Shape, pos: Point, mov: Point<isize>) -> Result<Point<isize>, ()> {
		let pos_new: Point<isize> = Point::from(pos) + mov;

		if pos_new.y < 0 || pos_new.x < 0 {
			return Err(());
		}

		let pos_new: Point = pos_new.into();

		for shape_point in shape.iter() {
			let point = pos_new + shape_point;
			if point.x >= TOWER_WIDTH || self.grid[point] == true {
				return Err(());
			}
		}

		Ok(mov)
	}

	fn place_shape(&mut self, shape: &Shape, pos: Point) {
		for shape_point in shape.iter() {
			self.grid[pos + shape_point] = true;
		}

		self.height = self.height.max(pos.y + shape.height());
		self.rocks_count += 1;
	}

	fn maybe_fast_advance(&mut self, rocks_limit: usize, shape_n: usize, mov_n: usize) {
		if let Some(state) = self.get_state(shape_n, mov_n) {
			if self.states.contains_key(&state) {
				let (prev_rocks, prev_height) = self.states[&state];
				let left = rocks_limit - self.rocks_count;
				let times = left / (self.rocks_count - prev_rocks);
				self.rocks_count += (self.rocks_count - prev_rocks) * times;
				self.height_sum = (self.height - prev_height) * times;
				self.states.clear();
			}

			self.states.insert(state, (self.rocks_count, self.height));
		}
	}

	fn get_state(&self, shape_n: usize, mov_n: usize) -> Option<State> {
		if self.height >= 20 {
			let top: Vec<bool> = self.grid[self.height - 20..self.height].iter()
				.flatten()
				.map(|v| *v)
				.collect();

			Some((shape_n, mov_n, top))
		} else {
			None
		}
	}
} // impl Tower

fn parse_input() -> Vec<Point<isize>> {
	let line = aoc::input::read_lines("day17").next().unwrap();
	line.chars().map(|ch| match ch {
		'<' => LEFT,
		'>' => RIGHT,
		_ => panic!()
	}).collect()
}

fn create_shapes() -> Vec<Shape> {
	vec![
		Shape(vec![
			vec![true, true, true, true]
		]),
		Shape(vec![
			vec![false, true, false],
			vec![true, true, true],
			vec![false, true, false]
		]),
		Shape(vec![
			vec![true, true, true],
			vec![false, false, true],
			vec![false, false, true]
		]),
		Shape(vec![
			vec![true],
			vec![true],
			vec![true],
			vec![true]
		]),
		Shape(vec![
			vec![true, true],
			vec![true, true]
		])
	]
}

impl Shape {
	fn iter(&self) -> impl Iterator<Item = Point> + '_ {
		self.0.iter_grid().filter(|(_, v)| **v).map(|(p, _)| p)
	}

	fn height(&self) -> usize {
		self.0.len()
	}
}