use adventofcode2022 as aoc;
use aoc::space_2D::Point;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Blizzard {
	vel: i32,
	pos0: i32
}

#[derive(Default)]
struct Grid {
	width: usize,
	height: usize,
	rows: Vec<Vec<Blizzard>>,
	cols: Vec<Vec<Blizzard>>,
}

struct State {
	pos: Point,
	minute: usize,
}

#[derive(Hash, PartialEq, Eq)]
struct StateCacheKey {
	pos: Point,
	rows_move: usize,
	cols_move: usize,
}

type StateCache = HashMap<StateCacheKey, usize>;
type StateCacheItem = (StateCacheKey, usize);

fn main() {
	let grid =parse_input("day24");

	let mut minutes = solve(&grid, grid.start_pos(), grid.end_pos(), 0);
	println!("Part 1: min steps {}", minutes);

	minutes = solve(&grid, grid.end_pos(), grid.start_pos(), minutes);
	minutes = solve(&grid, grid.start_pos(), grid.end_pos(), minutes);
	println!("Part 2: min steps {}", minutes);
}

fn solve(grid: &Grid, start_pos: Point, goal_pos: Point, start_minute: usize) -> usize {
	let mut queue = VecDeque::from([State {pos: start_pos, minute: start_minute}]);
	let mut min_minutes = usize::MAX;
	let mut seen_states = HashMap::new();

	while let Some(state) = queue.pop_front() {
		let minute = state.minute + 1;
		for pos in grid.next_positions(state.pos, minute) {
			let state_cache = (StateCacheKey::new(pos, minute, &grid), minute);

			if better_prev_state_exists(&seen_states, &state_cache) {
				continue;
			}

			// if minute + manhattan_dist(pos, grid.goal_adj(goal_pos)) + 1 >= min_minutes {
			// 	continue;
			// }

			if pos == goal_pos {
				if minute < min_minutes {
					min_minutes = minute;
				}
				continue;
			}

			queue.push_back(State {pos, minute});
			seen_states.insert(state_cache.0, state_cache.1);
		}
	}

	min_minutes
}

fn better_prev_state_exists(seen_states: &StateCache, state_cache: &StateCacheItem) -> bool {
	if let Some(prev_minute) = seen_states.get(&state_cache.0) {
		return *prev_minute <= state_cache.1;
	}
	false
}

fn manhattan_dist(orig: Point, goal: Point) -> usize {
	orig.x.abs_diff(goal.x) + orig.y.abs_diff(goal.y)
}

impl Grid {
	fn next_positions(&self, pos: Point, minute: usize) -> impl Iterator<Item = Point> + '_ {	
		[(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)].into_iter()
			.filter_map(move |inc| self.make_move(pos, inc))
			.filter(move |pos_next| !self.have_blizzard(*pos_next, minute))
	}

	fn make_move(&self, pos: Point, inc: (isize, isize)) -> Option<Point> {
		let (y, x) = (pos.y as isize, pos.x as isize);
		let (height, width) = (self.height as isize, self.width as isize);
		let start_adj = Point {y: 0usize, x: 0usize};
		let end_adj = Point {y: self.height - 1, x: self.width - 1};

		if (0..height).contains(&(y + inc.0)) && (0..width).contains(&(x + inc.1)) {
			Some(pos.add_signed(inc.into()))
		} else if (pos == self.start_pos() && inc == (0, 0)) || (pos == start_adj && inc == (-1, 0)) {
			Some(self.start_pos())
		} else if pos == self.start_pos() && inc == (1, 0) {
			Some(start_adj)
		} else if (pos == self.end_pos() && inc == (0, 0)) || (pos == end_adj && inc == (1, 0)) {
			Some(self.end_pos())
		} else if pos == self.end_pos() && inc == (-1, 0) {
			Some(end_adj)
		} else {
			None
		}
	}

	fn have_blizzard(&self, pos: Point, minute: usize) -> bool {
		if pos == self.start_pos() || pos == self.end_pos() {
			return false;
		}
		self.cols[pos.x].iter().any(|blizzard| pos.y == blizzard.calc_move(minute, self.height))
		|| self.rows[pos.y].iter().any(|blizzard| pos.x == blizzard.calc_move(minute, self.width))
	}

	fn start_pos(&self) -> Point {
		Point {y: self.height * 2, x: self.width * 2}  // fake point
	}

	fn end_pos(&self) -> Point {
		Point {y: self.height * 4, x: self.width * 4}  // fake point
	}
}

impl Blizzard {
	fn calc_move(&self, minute: usize, path_size: usize) -> usize {
		let pos = self.pos0 + minute as i32 * self.vel;
		if pos >= 0 {
			pos as usize % path_size
		} else {
			((pos % path_size as i32) + path_size as i32) as usize
		}
	}
}

impl StateCacheKey {
	fn new(pos: Point, minute: usize, grid: &Grid) -> Self {
		Self {
			pos,
			rows_move: minute % grid.width,
			cols_move: minute % grid.height
		}
	}
}

fn parse_input(day_xx: &str) -> Grid {
	let mut grid: Grid = Default::default();

	let lines = aoc::input::read_lines(day_xx).into_iter()
		.skip(1)
		.enumerate()
		.take_while(|(_, l)| !l.starts_with("##"));
	
	for (row, line) in lines {
		let line = &line[1..line.len() - 1];
		grid.height = row + 1;
		grid.width  = line.len();
		grid.rows.push(Vec::new());

		for (col, ch) in line.chars().enumerate() {
			if let None = grid.cols.get(col) {
				grid.cols.push(Vec::new());
			}

			match ch {
				'>' => grid.rows[row].push(Blizzard{vel: 1, pos0: col as i32}),
				'<' => grid.rows[row].push(Blizzard{vel: -1, pos0: col as i32}),
				'v' => grid.cols[col].push(Blizzard{vel: 1, pos0: row as i32}),
				'^' => grid.cols[col].push(Blizzard{vel: -1, pos0: row as i32}),
				'.' => (),
				_   => panic!("unexpected char {}", ch),
			}
		}
	}

	grid
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_reverse() {
		let grid = parse_input("day24-test");
		assert_eq!(solve(&grid, grid.end_pos(), grid.start_pos(), 0), 23);
	}

	#[test]
	fn test_part2() {
		let grid = parse_input("day24-test");
		let mut minutes = solve(&grid, grid.start_pos(), grid.end_pos(), 0);
		minutes = solve(&grid, grid.end_pos(), grid.start_pos(), minutes);
		minutes = solve(&grid, grid.start_pos(), grid.end_pos(), minutes);
		assert_eq!(minutes, 54);
	}
}