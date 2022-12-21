use adventofcode2022 as aoc;
use std::str::FromStr;
use std::collections::HashMap;
use Op::*;

struct Monkey(String, Op);

#[derive(Clone)]
enum Op {
	Num(i64),
	Sum(String, String),
	Sub(String, String),
	Mul(String, String),
	Div(String, String)
}

fn main() {
	let mut unsolved = parse_input("day21");
	let solved = extract_solved(&mut unsolved);
	println!("Part 1: root = {}", part1(unsolved.clone(), solved.clone()));
	println!("Part 2: humn = {}", part2(unsolved, solved));
}

fn extract_solved(unsolved: &mut HashMap<String, Op>) -> HashMap<String, i64> {
	let mut solved = HashMap::new();

	unsolved.retain(|monkey, op| {
		if let Num(n) = op {
			solved.insert(monkey.clone(), *n);
			return false;
		}
		true
	});

	solved
}

fn part1(mut unsolved: HashMap<String, Op>, mut solved: HashMap<String, i64>) -> i64 {
	while !unsolved.is_empty() && !solved.contains_key("root") {
		solve_cycle(&mut unsolved, &mut solved);
	}

	solved["root"]
}

fn part2(mut unsolved: HashMap<String, Op>, mut solved: HashMap<String, i64>) -> i64 {
	let root = unsolved["root"]
		.get_operands()
		.map(|(m1, m2)| (m1.to_string(), m2.to_string()))
		.unwrap();
	solved.remove("humn");

	let mut solved_count = 0;
	while solved_count != solved.len() {
		solved_count = solved.len();
		solve_cycle(&mut unsolved, &mut solved);
	}

	if solved.contains_key(&root.0) {
		let val = solved[&root.0];
		solved.insert(root.1.clone(), val);
	} else {
		let val = solved[&root.1];
		solved.insert(root.0.clone(), val);
	};

	while !solved.contains_key("humn") {
		solve_cycle_inverse(&mut unsolved, &mut solved);
	}

	solved["humn"]
}

fn solve_cycle(unsolved: &mut HashMap<String, Op>, solved: &mut HashMap<String, i64>) {
	unsolved.retain(|monkey, op| {
		let (m1, m2) = op.get_operands().unwrap();

		let (n1, n2) = match (solved.get(m1), solved.get(m2)) {
			(Some(n1), Some(n2)) => (*n1, *n2),
			_ => return true, // retain
		};

		match op {
			Sum(..) => solved.insert(monkey.clone(), n1 + n2),
			Sub(..) => solved.insert(monkey.clone(), n1 - n2),
			Mul(..) => solved.insert(monkey.clone(), n1 * n2),
			Div(..) => solved.insert(monkey.clone(), n1 / n2),
			_ => panic!(),
		};
		
		false  // remove
	});
}

fn solve_cycle_inverse(unsolved: &mut HashMap<String, Op>, solved: &mut HashMap<String, i64>) {
	unsolved.retain(|monkey, op| {
		let res = match solved.get(monkey) {
			Some(v) => v,
			None => return true, // retain
		};

		let (m1, m2) = op.get_operands().unwrap();
		match (solved.get(m1), solved.get(m2)) {
			(Some(n1), None) => {
				let n2 = match op {
					Sum(..) => res - *n1,
					Sub(..) => *n1 - res,
					Mul(..) => res / *n1,
					Div(..) => *n1 / res,
					_ => panic!(),
				};
				solved.insert(m2.to_string(), n2);
				return false;  // remove
			},
			(None, Some(n2)) => {
				let n1 = match op {
					Sum(..) => res - *n2,
					Sub(..) => res + *n2,
					Mul(..) => res / *n2,
					Div(..) => res * *n2,
					_ => panic!(),
				};
				solved.insert(m1.to_string(), n1);
				return false;  // remove
			},
			_ => {
				return true;  // retain
			},
		}
	});
}

fn parse_input(day_xx: &str) -> HashMap<String, Op> {
	aoc::input::parse_lines::<Monkey>(day_xx).map(|m| (m.0, m.1)).collect()
}

impl Op {
	fn get_operands(&self) -> Option<(&str, &str)> {
		match self {
			Sum(m1, m2) | Sub(m1, m2) | Mul(m1, m2) | Div(m1, m2 ) => Some((m1, m2)),
			_ => None,
		}
	}
}

impl FromStr for Monkey {
	type Err = aoc::input::ParseAoCInputError<Op>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let monkey = s[..4].to_string();
		let words: Vec<&str> = s[6..].split_ascii_whitespace().collect();
		let op = match words.get(1) {
			Some(&"+") => Sum(words[0].to_string(), words[2].to_string()),
			Some(&"-") => Sub(words[0].to_string(), words[2].to_string()),
			Some(&"*") => Mul(words[0].to_string(), words[2].to_string()),
			Some(&"/") => Div(words[0].to_string(), words[2].to_string()),
			None => Num(words[0].parse()?),
			_ => panic!()
		};
		Ok(Monkey(monkey, op))
	}
}