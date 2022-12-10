
pub mod input {
	use std::fs::File;
	use std::io::{BufRead, BufReader};
	use std::str::FromStr;
	use std::fmt;

	pub fn read_lines(day_xx: &str) -> impl Iterator<Item = String> {
		let filename = format!("input/{}.txt", day_xx);
		let f = File::open(filename).unwrap();
		let reader = BufReader::new(f);
		reader.lines().map(|l| l.unwrap())
	}

	pub fn parse_lines<T>(day_xx: &str) -> impl Iterator<Item = T>
	where
		T: FromStr,
		<T as FromStr>::Err: fmt::Debug
	{
		read_lines(day_xx).map(|l| l.parse().unwrap())
	}

	pub fn read_tokens_split_str<'a, T>(day_xx: &str, delim: &'a str)
		-> impl Iterator<Item = Vec<T>> + 'a
	where
		T: FromStr,
		<T as FromStr>::Err: fmt::Debug
	{
		read_lines(day_xx).map(move |line| {
			line.split(delim)
			    .map(|token| token.parse::<T>().unwrap())
			    .collect()
		})
	}

	pub fn read_tokens_split_chars<'a, T>(day_xx: &str, delim: &'a [char])
		-> impl Iterator<Item = Vec<T>> + 'a
	where
		T: FromStr,
		<T as FromStr>::Err: fmt::Debug
	{
		read_lines(day_xx).map(move |line| {
			line.split(delim)
			    .map(|token| token.parse::<T>().unwrap())
			    .collect()
		})
	}

	#[derive(Debug)]
	pub struct ParseAoCInputError {
		instr: String
	}

	impl ParseAoCInputError {
		pub fn new(instr: &str) -> Self {
			ParseAoCInputError { instr: String::from(instr) }
		}
	}

	impl fmt::Display for ParseAoCInputError {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "Invalid instruction '{}'", &self.instr)
		}
	}
}
