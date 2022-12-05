use adventofcode2022 as aoc;

#[derive(Debug)]
struct Move {
	qty: u32,
	from: usize,
	to: usize
}

fn main() {
	let lines: Vec<String> = aoc::input::read_lines("day05").collect();
	let split_idx = lines.iter().position(|l| l == " 1   2   3   4   5   6   7   8   9 ").unwrap();

	let stacks1 = parse_stacks(&lines[..split_idx]);
	let stacks2 = parse_stacks(&lines[..split_idx]);
	let moves = parse_moves(&lines[split_idx + 2..]);
	
	part1(stacks1, &moves);
	part2(stacks2, &moves);
}

fn part1(mut stacks: Vec<Vec<char>>, moves: &Vec<Move>) {
	for mov in moves {
		for _ in 0..mov.qty {
			let elem = stacks[mov.from].pop().unwrap();
			stacks[mov.to].push(elem);
		}
	}

	println!("Part 1: top elements '{}'", top_elems(&stacks));
}

fn part2(mut stacks: Vec<Vec<char>>, moves: &Vec<Move>) {
	for mov in moves {
		let pop_idx = stacks[mov.from].len() - mov.qty as usize;
		let elems: Vec<char> = stacks[mov.from].drain(pop_idx..).collect();
		stacks[mov.to].extend(elems);
	}

	println!("Part 2: top elements '{}'", top_elems(&stacks));
}

fn top_elems(stacks: &Vec<Vec<char>>) -> String {
	stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn parse_stacks(lines: &[String]) -> Vec<Vec<char>> {
	let mut stacks = vec![vec![]; 9];

	for line in lines.iter().rev() {
		for i in 0..9 {
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
	fn test1() {
		let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
		let movs = [
			String::from("move 1 from 2 to 1"),
			String::from("move 3 from 1 to 3"),
			String::from("move 2 from 2 to 1"),
			String::from("move 1 from 1 to 2")
		];
		let movs = parse_moves(&movs);
		part1(stacks, &movs);
	}
}