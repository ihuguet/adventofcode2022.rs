
pub mod input {
	use std::fs::File;
	use std::io::{BufRead, BufReader};

	pub fn read_lines(filename: &str) -> impl Iterator<Item = String> {
		let f = File::open(filename).unwrap();
		let reader = BufReader::new(f);
		reader.lines().map(|l| l.unwrap())
	}

	pub fn read_tokens(filename: &str, delim: &str) ->
		impl Iterator<Item = Vec<String>>
	{
		let lines_iter = read_lines(filename);
		let delim = String::from(delim);
		lines_iter.map(move |l| {
			l.split(&delim)
				.map(String::from)
				.collect()
		})
	}
}
