use adventofcode2022 as aoc;
use std::collections::VecDeque;
use std::cmp;

#[derive(Clone)]
struct Monkey {
	items: VecDeque<u64>,
	op: Op,
	test_div_by: u64,
	next_true: usize,
	next_false: usize,
	inspections: usize
}

#[derive(Clone, Copy)]
enum Op {
	Sum(u64),
	Mul(u64),
	Sq,
}

fn main() {
	let monkeys = parse_input("day11");
	println!("Part 1: monkey business {}", solve(monkeys.clone(), 20, 3));
	println!("Part 2: monkey business {}", solve(monkeys, 10000, 1));
}
	
fn solve(mut monkeys: Vec<Monkey>, rounds: usize, item_div: u64) -> usize {
	let common_div: u64 = monkeys.iter().map(|m| m.test_div_by).product();

	for _i in 0..rounds {
		for i in 0..monkeys.len() {
			monkeys[i].inspections += monkeys[i].items.len();
			let &Monkey {op, test_div_by, next_true, next_false, ..} = &monkeys[i];

			while let Some(mut item) = monkeys[i].items.pop_front() {
				item = op.calc(item) / item_div % common_div;

				if item % test_div_by == 0 {
					monkeys[next_true].items.push_back(item)
				} else {
					monkeys[next_false].items.push_back(item)
				};
			}
		}
	}

	let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
	inspections.sort_by_key(|v| cmp::Reverse(*v));
	inspections[0] * inspections[1]
}

impl Op {
	fn calc(&self, val: u64) -> u64 {
		match self {
			Op::Sum(val2) => val + *val2,
			Op::Mul(val2) => val * *val2,
			Op::Sq => val * val,
		}
	}
}

fn parse_input(day_xx: &str) -> Vec<Monkey> {
	let input = aoc::input::read_lines(day_xx).collect::<Vec<_>>();
	input.chunks(7).map(parse_monkey_data).collect()
}

fn parse_monkey_data(monkey_data: &[String]) -> Monkey {
	let items = monkey_data[1][18..].split(", ").map(|v| v.parse().unwrap()).collect();
	let op = &monkey_data[2][23..24];
	let val = &monkey_data[2][25..];
	let test_div_by = monkey_data[3][21..].parse().unwrap();
	let next_true = monkey_data[4][29..].parse().unwrap();
	let next_false = monkey_data[5][30..].parse().unwrap();

	let op = match (op, val) {
		("*", "old") => Op::Sq,
		("*", num) => Op::Mul(num.parse().unwrap()),
		("+", num) => Op::Sum(num.parse().unwrap()),
		_ => panic!(),
	};

	Monkey {items, op, test_div_by, next_true, next_false, inspections: 0}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let monkeys = parse_input("day11-test");
		assert_eq!(solve(monkeys, 20, 3), 10605);
	}

	#[test]
	fn test_part2() {
		let monkeys = parse_input("day11-test");
		assert_eq!(solve(monkeys, 10000, 1), 2713310158);
	}
}