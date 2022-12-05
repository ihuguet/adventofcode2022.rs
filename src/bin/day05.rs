use adventofcode2022 as aoc;

#[derive(Debug)]
struct Move {
	qty: u32,
	from: usize,
	to: usize
}

fn main() {
	let (stacks, moves) = parse_input("day05");
	println!("Part 1: top elements '{}'", solve(stacks.clone(), &moves, true));
	println!("Part 2: top elements '{}'", solve(stacks, &moves, false));
}

fn solve(mut stacks: Vec<Vec<char>>, moves: &Vec<Move>, one_by_one: bool) -> String {
	for mov in moves {
		let pop_idx = stacks[mov.from].len() - mov.qty as usize;
		let mut elems: Vec<char> = stacks[mov.from].drain(pop_idx..).collect();
		if one_by_one {
			elems.reverse();
		}
		stacks[mov.to].extend(elems);
	}
	top_elems(&stacks)
}

fn top_elems(stacks: &Vec<Vec<char>>) -> String {
	stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn parse_input(day_xx: &str) -> (Vec<Vec<char>>, Vec<Move>) {
	let lines: Vec<String> = aoc::input::read_lines(day_xx).collect();
	let split_idx = lines.iter().position(|l| l.starts_with(" 1")).unwrap();
	let stacks_count = lines[split_idx].split_whitespace().count();
	(
		parse_stacks(&lines[..split_idx], stacks_count),
		parse_moves(&lines[split_idx + 2..])
	)
}

fn parse_stacks(lines: &[String], stacks_count: usize) -> Vec<Vec<char>> {
	let mut stacks = vec![vec![]; stacks_count];

	for line in lines.iter().rev() {
		for i in 0..stacks_count {
			let ch = line.chars().nth(1 + i * 4).unwrap();
			if ch != ' ' {
				stacks[i].push(ch);
			}
		}
	}

	stacks
}

fn parse_moves(lines: &[String]) -> Vec<Move> {
	lines.iter().map(|line| {
		let words: Vec<&str> = line.split_whitespace().collect();
		Move {
			qty: words[1].parse().unwrap(),
			from: words[3].parse::<usize>().unwrap() - 1,
			to: words[5].parse::<usize>().unwrap() - 1
		}
	}).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let (stacks, moves) = parse_input("day05-test");
		assert_eq!(solve(stacks, &moves, true), "CMZ");
	}

	#[test]
	fn test_part2() {
		let (stacks, moves) = parse_input("day05-test");
		assert_eq!(solve(stacks, &moves, false), "MCD");
	}
}