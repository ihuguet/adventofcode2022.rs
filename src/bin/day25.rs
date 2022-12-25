use adventofcode2022 as aoc;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Display;

struct Snafu(i64);

const SNAFU_TO_DEC: [(char, i64); 5] = [('=', -2), ('-', -1), ('0', 0), ('1', 1), ('2', 2)];
const DEC_TO_SNAFU: [(i64, (char, i64)); 6] = [(0, ('0', 0)), (1, ('1', 0)), (2, ('2', 0)), (3, ('=', 1)), (4, ('-', 1)), (5, ('0', 1))];

fn main() {
	let sum = aoc::input::parse_lines::<Snafu>("day25").fold(0, |acc, n| acc + n.0);
	println!("SNAFU sum: {}", Snafu(sum));
}

impl FromStr for Snafu {
	type Err = aoc::input::ParseAoCInputError<Snafu>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let conversion = HashMap::from(SNAFU_TO_DEC);
		let mut base = 1;
		let result = s.chars().rev().fold(0, |mut acc, ch| {acc += base * conversion[&ch]; base *= 5; acc});
		Ok(Snafu(result))
	}
}

impl Display for Snafu {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let conversion = HashMap::from(DEC_TO_SNAFU);
		let mut result = Vec::new();
		let mut num = self.0;
		let mut carry = 0;

		while num != 0 || carry != 0{
			let digit = num % 5 + carry;
			result.push(conversion[&digit].0);
			carry = conversion[&digit].1;
			num /= 5;
		}
		let result: String = result.iter().rev().collect();

		write!(f, "{}", result)
	}
}