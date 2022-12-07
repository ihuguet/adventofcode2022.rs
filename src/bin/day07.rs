use adventofcode2022 as aoc;
use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::Borrow;
use core::hash::Hash;

#[derive(Debug, PartialEq, Default)]
struct Dir {
	dirs: HashMap<String, Dir>,
	files: HashMap<String, u64>,
	size_cache: RefCell<Option<u64>>
}

fn main() {
	let root = parse_input("day07");
	println!("Part 1: size {}", sum_lte_100k(&root));
	println!("Part 2: size {}", smallest_size_to_free(&root, 40_000_000).unwrap())
}

fn sum_lte_100k(dir: &Dir) -> u64 {
	let own = dir.calc_size();
	let children: u64 = dir.dirs.values().map(sum_lte_100k).sum();
	children + if own <= 100_000 { own } else { 0 }
}

fn smallest_size_to_free(root: &Dir, size: u64) -> Option<u64> {
	let size_to_free = root.calc_size() - size;
	smallest_size_gte(root, size_to_free)
}

fn smallest_size_gte(root: &Dir, size: u64) -> Option<u64> {
	let mut min = if root.calc_size() >= size {
		Some(root.calc_size())
	} else {
		None
	};

	for child_size in root.dirs.values().filter_map(|d| smallest_size_gte(d, size)) {
		if min.is_none() || child_size < min.unwrap() {
			min = Some(child_size)
		}
	}

	min
}

fn parse_input(day_xx: &str) -> Dir {
	let lines = aoc::input::read_tokens_split_chars::<String>(day_xx, &[' ']);

	let mut root = Dir::new();
	let mut path = vec![];

	for words in lines {
		match words[0].as_str() {
			"$" => if words[1] == "cd" {
				if words[2] == ".." {
					path.pop();
				} else {
					path.push(words[2].to_string());
				}
			},
			"dir" => root.path(&path[1..]).insert_dir(&words[1]),
			_ => root.path(&path[1..]).insert_file(&words[1], words[0].parse().unwrap()),
		}
	}

	root
}

impl Dir {
	fn new() -> Dir {
		Default::default()
	}

	fn insert_dir(&mut self, name: &str) {
		self.dirs.insert(name.to_string(), Dir::new());
		*self.size_cache.get_mut() = None;
	}

	fn insert_file(&mut self, name: &str, size: u64) {
		self.files.insert(name.to_string(), size);
		*self.size_cache.get_mut() = None;
	}

	fn path<S>(&mut self, path: &[S]) -> &mut Dir
	where
		S: Hash + Eq + std::fmt::Display,
		String: Borrow<S>
	{
		let mut current = self;
		for dir in path {
			let next = current.dirs.get_mut(dir).expect(&format!("missing dir {}", dir));
			current = next;
		}
		current
	}

	fn calc_size(&self) -> u64 {
		self.size_cache.borrow_mut().get_or_insert_with(|| {
			self.files.values().copied().sum::<u64>()
			+ self.dirs.values().map(Dir::calc_size).sum::<u64>()
		}).clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_tree() {
		let parsed = parse_input("day07-test");

		let mut root = Dir::new();
		root.insert_dir("a");
		root.insert_dir("d");
		root.insert_file("b.txt", 14848514);
		root.insert_file("c.dat",8504156);

		let dir = root.dirs.get_mut("a").unwrap();
		dir.insert_dir("e");
		dir.insert_file("f", 29116);
		dir.insert_file("g", 2557);
		dir.insert_file("h.lst", 62596);

		let dir = dir.dirs.get_mut("e").unwrap();
		dir.insert_file("i", 584);

		let dir = root.dirs.get_mut("d").unwrap();
		dir.insert_file("j", 4060174);
		dir.insert_file("d.log", 8033020);
		dir.insert_file("d.ext", 5626152);
		dir.insert_file("k", 7214296);

		assert_eq!(parsed, root);
	}

	#[test]
	fn test_part1() {
		let mut root = parse_input("day07-test");
		assert_eq!(sum_lte_100k(&mut root), 95437);
	}

	#[test]
	fn test_part2() {
		let mut root = parse_input("day07-test");
		assert_eq!(smallest_size_to_free(&mut root, 40_000_000), Some(24933642));
	}
}