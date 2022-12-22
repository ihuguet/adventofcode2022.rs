use adventofcode2022 as aoc;
use aoc::space_2D::{Grid, VecGrid};
use std::collections::BTreeMap;
use Orientation::*;

type Point = aoc::space_2D::Point<isize>;
type WrapFn = dyn Fn(&VecGrid<u8>, Point, Orientation) -> (Point, Orientation);

#[derive(Clone)]
struct Traveler {
	pos: Point,
	orientation: Orientation
}

#[derive(Clone)]
struct Indications {
	str: String,
	pos: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Orientation {
	RIGHT, DOWN, LEFT, UP
}

fn main() {
	let (map, dirs) = parse_input("day22");
	println!("Part1: result {}", solve(&map, dirs.clone(), &wrap));

	let mappings = create_cube_wrap();
	let wrap_cube = move |_m: &Vec<Vec<u8>>, pos: Point, orientation: Orientation| -> (Point, Orientation) {
		mappings[&(pos, orientation)]
	};
	println!("Part2: result {}", solve(&map, dirs, &wrap_cube));
}

fn solve(map: &VecGrid<u8>, mut indications: Indications, wrap_fn: &WrapFn) -> isize {
	let col_start = map[0].iter().position(|ch| *ch == b'.').unwrap() as isize;
	let mut traveler = Traveler {pos: (0, col_start).into(), orientation: RIGHT};

	while !indications.done() {
		if let Some(mut steps) = indications.get_next_move_steps() {
			while steps > 0 {
				let next = move_maybe_wrap(map, &traveler, wrap_fn);
				if map[next.pos] == b'#' {
					break;
				}
				traveler = next;
				steps -= 1;
			}
		}

		if let Some(turn) = indications.get_next_turn() {
			traveler.orientation = traveler.orientation.turn(turn);
		}
	}

	1000 * (traveler.pos.y + 1) + 4 * (traveler.pos.x + 1) + traveler.orientation as isize
}

fn move_maybe_wrap(map: &VecGrid<u8>, traveler: &Traveler, wrap_fn: &WrapFn) -> Traveler {
	let mut pos = traveler.pos + traveler.orientation.get_move_dir();
	let mut orientation = traveler.orientation;

	if map.get_point(pos).is_none() || map[pos] == b' ' {
		(pos, orientation) = wrap_fn(&map, pos, orientation);
	}

	Traveler {pos, orientation}
}

impl Indications {
	fn done(&self) -> bool {
		self.pos >= self.str.len()
	}

	fn get_next_move_steps(&mut self) -> Option<isize> {
		let start = self.pos;
		let bytes = self.str.as_bytes();
		while !self.done() && bytes[self.pos].is_ascii_digit() {
			self.pos += 1;
		}
		if start != self.pos {
			Some(self.str[start..self.pos].parse().unwrap())
		} else {
			None
		}
	}

	fn get_next_turn(&mut self) -> Option<char> {
		if !self.done() {
			let i = self.pos;
			self.pos += 1;
			Some(self.str.as_bytes()[i] as char)
		} else {
			None
		}
	}
}

impl Orientation {
	fn get_move_dir(&self) -> Point {
		Point::from(match *self {
			RIGHT => (0, 1),
			DOWN  => (1, 0),
			LEFT  => (0, -1),
			UP    => (-1, 0)
		})
	}

	fn turn(&self, turn: char) -> Orientation {
		match (self, turn) {
			(RIGHT, 'R') => DOWN,
			(RIGHT, 'L') => UP,
			(DOWN, 'R')  => LEFT,
			(DOWN, 'L')  => RIGHT,
			(LEFT, 'R')  => UP,
			(LEFT, 'L')  => DOWN,
			(UP, 'R')    => RIGHT,
			(UP, 'L')    => LEFT,
			_ => panic!()
		}
	}
}

fn wrap(map: &VecGrid<u8>, pos: Point, orientation: Orientation) -> (Point, Orientation) {
	let (y, x) = (pos.y as usize, pos.x as usize);
	let pos = match orientation {
		RIGHT => (y, map[y].iter().position(|ch| *ch != b' ').unwrap()),
		DOWN  => (map.iter().position(|row| row[x] != b' ').unwrap(), x),
		LEFT  => (y, map[y].iter().rposition(|ch| *ch != b' ').unwrap()),
		UP    => (map.iter().rposition(|row| row[x] != b' ').unwrap(), x)
	};
	(Point::from((pos.0 as isize, pos.1 as isize)), orientation)
}

fn create_cube_wrap() -> BTreeMap<(Point, Orientation), (Point, Orientation)> {
	let mut mappings: BTreeMap<(Point, Orientation), (Point, Orientation)> = BTreeMap::new();

	for n in 0..50 {
		mappings.insert(((n, 49).into(), LEFT), ((149 - n, 0).into(), RIGHT));
		mappings.insert(((149 - n, -1).into(), LEFT), ((n, 50).into(), RIGHT));

		mappings.insert(((-1, 50 + n).into(), UP), ((150 + n, 0).into(), RIGHT));
		mappings.insert(((150 + n, -1).into(), LEFT), ((0, 50 + n).into(), DOWN));
		
		mappings.insert(((-1, 100 + n).into(), UP), ((199, n).into(), UP));
		mappings.insert(((200, n).into(), DOWN), ((0, 100 + n).into(), DOWN));

		mappings.insert(((n, 150).into(), RIGHT), ((149 - n, 99).into(), LEFT));
		mappings.insert(((149 - n, 100).into(), RIGHT), ((n, 149).into(), LEFT));

		mappings.insert(((50, 100 + n).into(), DOWN), ((50 + n, 99).into(), LEFT));
		mappings.insert(((50 + n, 100).into(), RIGHT), ((49, 100 + n).into(), UP));

		mappings.insert(((50 + n, 49).into(), LEFT), ((100, n).into(), DOWN));
		mappings.insert(((99, n).into(), UP), ((50 + n, 50).into(), RIGHT));

		mappings.insert(((150, 50 + n).into(), DOWN), ((150 + n, 49).into(), LEFT));
		mappings.insert(((150 + n, 50).into(), RIGHT), ((149, 50 + n).into(), UP));
	}

	mappings
}

fn parse_input(day_xx: &str) -> (VecGrid<u8>, Indications) {
	let mut lines = aoc::input::read_lines(day_xx);

	let mut max_len = 0;
	let mut map = lines.by_ref().map_while(|line| {
		if !line.is_empty() {
			max_len = max_len.max(line.len());
			Some(line.into_bytes())
		} else {
			None
		}
	}).collect::<Vec<_>>();

	for row in &mut map {
		row.resize(max_len, b' ');
	}

	let dirs = lines.next().unwrap();

	(map, Indications {str: dirs, pos: 0})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let (map, dirs) = parse_input("day22-test");
		assert_eq!(solve(&map, dirs, &wrap), 6032);
	}
}