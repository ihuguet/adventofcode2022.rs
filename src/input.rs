use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
use std::fmt;

type Err<T> = <T as FromStr>::Err;

pub fn read_lines(day_xx: &str) -> impl Iterator<Item = String> {
	let filename = format!("input/{}.txt", day_xx);
	let f = File::open(filename).unwrap();
	let reader = BufReader::new(f);
	reader.lines().map(|l| l.unwrap())
}

pub fn parse_lines<T>(day_xx: &str) -> impl Iterator<Item = T>
where
	T: FromStr,
	Err<T>: fmt::Debug
{
	read_lines(day_xx).map(|l| l.parse().unwrap())
}

pub fn parse_lines_safe<T>(day_xx: &str) -> impl Iterator<Item = Result<T, Err<T>>>
where
	T: FromStr
{
	read_lines(day_xx).map(|l| l.parse())
}

pub fn read_tokens_split_str<T>(day_xx: &str, delim: &str) -> impl Iterator<Item = Vec<T>>
where
	T: FromStr,
	Err<T>: fmt::Debug
{
	let delim = delim.to_string();
	read_lines(day_xx).map(move |line| {
		line.split(&delim)
			.map(|token| token.parse::<T>().unwrap())
			.collect()
	})
}

pub fn read_tokens_safe_split_str<T>(day_xx: &str, delim: &str)
	-> impl Iterator<Item = Result<Vec<T>, Err<T>>>
where
	T: FromStr
{
	let delim = delim.to_string();
	read_lines(day_xx).map(move |line| {
		line.split(&delim)
			.map(|token| token.parse::<T>())
			.collect()
	})
}

pub fn read_tokens_split_chars<T>(day_xx: &str, delim: &[char]) -> impl Iterator<Item = Vec<T>> 
where
	T: FromStr,
	Err<T>: fmt::Debug
{
	let delim = delim.to_vec();
	read_lines(day_xx).map(move |line| {
		line.split(delim.as_slice())
			.map(|token| token.parse::<T>().unwrap())
			.collect()
	})
}

pub fn read_tokens_safe_split_chars<T>(day_xx: &str, delim: &[char])
	-> impl Iterator<Item = Result<Vec<T>, Err<T>>>
where
	T: FromStr
{
	let delim = delim.to_vec();
	read_lines(day_xx).map(move |line| {
		line.split(delim.as_slice())
			.map(|token| token.parse::<T>())
			.collect()
	})
}

/// Error type that can be used by `impl FromStr for MyType`
pub struct ParseAoCInputError<T> {
	wrong_str: String,
	custom_msg: Option<String>,
	data_type: std::marker::PhantomData<T>,
}

impl<T> ParseAoCInputError<T> {
	pub fn new(wrong_str: &str) -> Self {
		ParseAoCInputError {
			wrong_str: String::from(wrong_str),
			custom_msg: None,
			data_type: std::marker::PhantomData,
		}
	}

	pub fn new_custom(custom_msg: &str) -> Self {
		ParseAoCInputError {
			wrong_str: String::new(),
			custom_msg: Some(String::from(custom_msg)),
			data_type: std::marker::PhantomData,
		}
	}
}

impl<T> fmt::Display for ParseAoCInputError<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(msg) = &self.custom_msg {
			write!(f, "{}", msg)
		} else {
			write!(f, "Can't parse '{}' to type '{}'", &self.wrong_str, std::any::type_name::<T>())
		}
	}
}

impl<T> fmt::Debug for ParseAoCInputError<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<Self as fmt::Display>::fmt(&self, f)
	}
}

impl<T> From<ParseIntError> for ParseAoCInputError<T> {
	fn from(err: ParseIntError) -> Self {
		let msg = format!("ParseIntError (kind {:?}) while trying to parse to type '{}'",
						err.kind(), std::any::type_name::<T>());
		ParseAoCInputError::new_custom(&msg)
	}
}
