use adventofcode2022 as aoc;

const DECRYPTION_KEY: i64 = 811589153;

fn main() {
	let nums = parse_input("day20");
	println!("Part 1: sum {}", solve(&nums, 1));

	let nums: Vec<i64> = nums.into_iter().map(|n| n * DECRYPTION_KEY).collect();
	println!("Part 2: sum {}", solve(&nums, 10));
}

fn solve(nums: &[i64], cycles: usize) -> i64 {
	let mut positions: Vec<usize> = (0..nums.len()).collect();
	
	for _ in 0..cycles {
		for i in 0..positions.len() {
			let pos = positions[i];
			let num = nums[i];
			let new_pos = pos_add(pos, num, nums.len());

			if new_pos > pos {
				positions.iter_mut()
					.filter(|p| **p > pos && **p <= new_pos)
					.for_each(|p| *p = pos_add(*p, -1, nums.len()));
			} else if new_pos < pos {
				positions.iter_mut()
					.filter(|p| **p >= new_pos && **p < pos)
					.for_each(|p| *p = pos_add(*p, 1, nums.len()));
			}
			positions[i] = new_pos;
		}
	}

	let pos_of_0 = nums.iter().position(|n| *n == 0).map(|idx| positions[idx]).unwrap();
	let i1000 = positions.iter().position(|pos| *pos == (pos_of_0 + 1000) % nums.len()).unwrap();
	let i2000 = positions.iter().position(|pos| *pos == (pos_of_0 + 2000) % nums.len()).unwrap();
	let i3000 = positions.iter().position(|pos| *pos == (pos_of_0 + 3000) % nums.len()).unwrap();
	nums[i1000] + nums[i2000] + nums[i3000]
}

fn pos_add(pos: usize, mov: i64, len: usize) -> usize {
	let len = len as i64;
	match pos as i64 + mov % (len - 1) { // moving (len - 1) makes going back to same pos
		p if p < 0  => (p + len - 1) as usize,  // wrapping needs to move an additional pos
		p if p >= len => (p - len + 1) as usize,  // wrapping needs to move an additional pos
		p => p as usize
	}
}

fn parse_input(day_xx: &str) ->  Vec<i64> {
	aoc::input::parse_lines::<i64>(day_xx).collect()
}
