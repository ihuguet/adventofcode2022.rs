use adventofcode2022 as aoc;
use aoc::space_2D::{Point, Grid, VecGrid};
use std::collections::HashMap;

const TOWER_WIDTH: usize = 7;
const RIGHT: Point<isize> = Point {y: 0, x: 1};
const LEFT: Point<isize> = Point {y: 0, x: -1};
const DOWN: Point<isize> = Point {y: -1, x: 0};

#[derive(PartialEq)]
struct Shape(Vec<Vec<bool>>);

fn main() {
	let jet_pattern = parse_input();
	let shapes = create_shapes();
	println!("Part 1: tower {} high", solve(&jet_pattern, &shapes, 2022));
	println!("Part 2: tower {} high", solve(&jet_pattern, &shapes, 1000000000000));
}

fn solve(jet_pattern: &str, shapes: &[Shape], limit: usize) -> usize {
	let mut tower = vec![vec![false; 7]; 4000];

	let mut jet_pattern_iter = jet_pattern.chars().to_owned().cycle();
	let mut shapes_iter = (0..shapes.len()).cycle().skip(1);

	let mut stopped_shapes = 0;
	let mut tower_height = 0;
	let mut shape = &shapes[0];
	let mut shape_n;
	let mut pos = Point::from((3usize, 2usize));

	let mut states = HashMap::new();
	let mut tower_height_sum = 0;

	while stopped_shapes < limit {

		let mov = jet_pattern_iter.next().unwrap();
		let mov_result = match mov {
			'<' => check_move(&tower, shape, pos, LEFT),
			'>' => check_move(&tower, shape, pos, RIGHT),
			ch => panic!("Unexpected '{}'", ch)
		};
		if let Ok(mov) = mov_result {
			pos = pos.add_signed(mov);
		}

		if let Ok(mov) = check_move(&tower, shape, pos, DOWN) {
			pos = pos.add_signed(mov);
		} else {
			place_shape(&mut tower, shape, pos);
			tower_height = tower_height.max(pos.y + shape.height());
			stopped_shapes += 1;

			shape_n = shapes_iter.next().unwrap();
			shape = &shapes[shape_n];
			pos = Point::from((tower_height + 3, 2));

			// fast advance?
			if let Some(state) = get_state(&tower, tower_height, shape_n) {
				if states.contains_key(&state) {
					let (prev_shapes, prev_height) = states[&state];
					let left = limit - stopped_shapes;
					let times = left / (stopped_shapes - prev_shapes);
					stopped_shapes += (stopped_shapes - prev_shapes) * times;
					tower_height_sum = (tower_height - prev_height) * times;
					states.clear();
				}

				states.insert(state, (stopped_shapes, tower_height));
			}

			grow_tower_for_height(&mut tower, pos.y + shape.height());
		}
	}

	tower_height + tower_height_sum
}

fn check_move(tower: &VecGrid<bool>, shape: &Shape, pos: Point, mov: Point<isize>) -> Result<Point<isize>, ()> {
	let pos_new: Point<isize> = Point::from(pos) + mov;

	if pos_new.y < 0 || pos_new.x < 0 {
		return Err(());
	}

	let pos_new: Point = pos_new.into();

	for shape_point in shape.iter() {
		let point = pos_new + shape_point;
		if point.x >= TOWER_WIDTH || tower[point] == true {
			return Err(());
		}
	}

	Ok(mov)
}

fn place_shape(tower: &mut VecGrid<bool>, shape: &Shape, pos: Point) {
	for shape_point in shape.iter() {
		tower[pos + shape_point] = true;
	}
}

fn grow_tower_for_height(tower: &mut VecGrid<bool>, height: usize) {
	if tower.len() < height {
		tower.reserve(tower.len());
		tower.extend(std::iter::repeat(vec![false; 7]).take(tower.len()));
	}
}

fn get_state(tower: &VecGrid<bool>, height: usize, shape_n: usize) -> Option<(usize, Vec<bool>)> {
	if height >= 100 {
		let top: Vec<bool> = tower[height - 100..height].iter().flatten().map(|v| *v).collect();
		Some((shape_n, top))
	} else {
		None
	}
}

fn parse_input() -> String{
	aoc::input::read_lines("day17").next().unwrap()
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