use adventofcode2022 as aoc;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
	x: i32,
	y: i32
}

fn main() {
	let moves = parse_input();
	println!("Part 1: tail visited {} positions", rope_move(&moves, 2));
	println!("Part 2: tail visited {} positions", rope_move(&moves, 10));
}

fn rope_move(moves: &Vec<(String, i32)>, rope_size: usize) -> usize {
	let mut rope = vec![Point{x: 0, y: 0}; rope_size];
	let mut tail_visited = HashSet::new();
	tail_visited.insert(Point{x: 0, y: 0});

	for (dir, num) in moves {
		for _ in 0..*num {
			let head = &mut rope[0];
			match dir.as_str() {
				"L" => head.y -= 1,
				"R" => head.y += 1,
				"U" => head.x += 1,
				"D" => head.x -= 1,
				_ => panic!("Wrong direction {}", dir),
			};

			for i in 0..rope_size - 1{
				let dx = rope[i].x - rope[i + 1].x;
				let dy = rope[i].y - rope[i + 1].y;
				if dx.abs() > 1 || dy.abs() > 1 {
					rope[i + 1].x += dx.signum();
					rope[i + 1].y += dy.signum();
				}
			}

			tail_visited.insert(rope[rope_size - 1].clone());
		}
	}

	tail_visited.len()
}

fn parse_input() -> Vec<(String, i32)> {
	aoc::input::read_lines("day09").map(|l| {
		let tokens: Vec<&str> = l.split(" ").collect();
		(tokens[0].to_string(), tokens[1].parse::<i32>().unwrap())
	}).collect()
}