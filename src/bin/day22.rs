use adventofcode2022 as aoc;
use aoc::space_2D::{Grid, VecGrid};
use std::collections::BTreeMap;
use Orientation::*;

type Point = aoc::space_2D::Point<isize>;
type WrapFn = dyn Fn(&VecGrid<u8>, Traveler) -> Traveler;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
	let wrap_cube = move |_: &_, traveler| mappings[&traveler];
	println!("Part2: result {}", solve(&map, dirs, &wrap_cube));
}

fn solve(map: &VecGrid<u8>, mut indications: Indications, wrap_fn: &WrapFn) -> isize {
	let col_start = map[0].iter().position(|ch| *ch == b'.').unwrap() as isize;
	let mut traveler = Traveler {pos: (0, col_start).into(), orientation: RIGHT};

	while !indications.done() {
		if let Some(steps) = indications.get_next_move_steps() {
			for _ in 0..steps {
				let next = move_maybe_wrap(map, &traveler, wrap_fn);
				if map[next.pos] == b'#' {
					break;
				}
				traveler = next;
			}
		}

		if let Some(turn) = indications.get_next_turn() {
			traveler.orientation = traveler.orientation.turn(turn);
		}
	}

	1000 * (traveler.pos.y + 1) + 4 * (traveler.pos.x + 1) + traveler.orientation as isize
}

fn move_maybe_wrap(map: &VecGrid<u8>, traveler: &Traveler, wrap_fn: &WrapFn) -> Traveler {
	let mut traveler = traveler.clone();
	traveler.pos += traveler.orientation.get_move_dir();

	if map.get_point(traveler.pos).is_none() || map[traveler.pos] == b' ' {
		traveler = wrap_fn(&map, traveler);
	}

	traveler
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

fn wrap(map: &VecGrid<u8>, mut traveler: Traveler) -> Traveler {
	let (y, x) = (traveler.pos.y as usize, traveler.pos.x as usize);
	let pos = match traveler.orientation {
		RIGHT => (y, map[y].iter().position(|ch| *ch != b' ').unwrap()),
		DOWN  => (map.iter().position(|row| row[x] != b' ').unwrap(), x),
		LEFT  => (y, map[y].iter().rposition(|ch| *ch != b' ').unwrap()),
		UP    => (map.iter().rposition(|row| row[x] != b' ').unwrap(), x)
	};
	traveler.pos = (pos.0 as isize, pos.1 as isize).into();
	traveler
}

fn create_cube_wrap() -> BTreeMap<Traveler, Traveler> {
	let mut mappings: BTreeMap<Traveler, Traveler> = BTreeMap::new();
	let tr = |y, x, orientation| Traveler {pos: Point::from((y, x)), orientation};

	for n in 0..50 {
		mappings.insert(tr(n, 49, LEFT), tr(149 - n, 0, RIGHT));
		mappings.insert(tr(149 - n, -1, LEFT), tr(n, 50, RIGHT));

		mappings.insert(tr(-1, 50 + n, UP), tr(150 + n, 0, RIGHT));
		mappings.insert(tr(150 + n, -1, LEFT), tr(0, 50 + n, DOWN));
		
		mappings.insert(tr(-1, 100 + n, UP), tr(199, n, UP));
		mappings.insert(tr(200, n, DOWN), tr(0, 100 + n, DOWN));

		mappings.insert(tr(n, 150, RIGHT), tr(149 - n, 99, LEFT));
		mappings.insert(tr(149 - n, 100, RIGHT), tr(n, 149, LEFT));

		mappings.insert(tr(50, 100 + n, DOWN), tr(50 + n, 99, LEFT));
		mappings.insert(tr(50 + n, 100, RIGHT), tr(49, 100 + n, UP));

		mappings.insert(tr(50 + n, 49, LEFT), tr(100, n, DOWN));
		mappings.insert(tr(99, n, UP), tr(50 + n, 50, RIGHT));

		mappings.insert(tr(150, 50 + n, DOWN), tr(150 + n, 49, LEFT));
		mappings.insert(tr(150 + n, 50, RIGHT), tr(149, 50 + n, UP));
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