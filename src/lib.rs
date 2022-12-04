
pub mod input {
	use std::fs::File;
	use std::io::{BufRead, BufReader};
	use std::str::FromStr;
	use std::fmt::Debug;

	pub fn read_lines(day_xx: &str) -> impl Iterator<Item = String> {
		let filename = format!("input/{}.txt", day_xx);
		let f = File::open(filename).unwrap();
		let reader = BufReader::new(f);
		reader.lines().map(|l| l.unwrap())
	}

	pub fn read_tokens_split_str<'a, T>(day_xx: &str, delim: &'a str)
		-> impl Iterator<Item = Vec<T>> + 'a
	where
		T: FromStr,
		<T as FromStr>::Err: Debug
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
		<T as FromStr>::Err: Debug
	{
		read_lines(day_xx).map(move |line| {
			line.split(delim)
			    .map(|token| token.parse::<T>().unwrap())
			    .collect()
		})
	}
}
