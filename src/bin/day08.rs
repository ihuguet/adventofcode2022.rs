use adventofcode2022 as aoc;

fn main() {
	let grid: Vec<Vec<u8>> = aoc::input::read_lines("day08").map(String::into_bytes).collect();
	println!("Part 1: visible trees {}", count_visibles(&grid));
	println!("Part 2: max views score {}", get_max_views_score(&grid));
}

fn count_visibles(grid: &Vec<Vec<u8>>) -> usize {
	let mut visibles = vec![vec![false; grid[0].len()]; grid.len()];

	for i in 0..grid.len() {
		let mut max_height = 0; // all ascii bytes are higher
		for j in 0..grid[i].len() {
			if grid[i][j] > max_height {
				visibles[i][j] = true;
				max_height = grid[i][j];
			}
		}

		max_height = 0;
		for j in (0..grid[i].len()).rev() {
			if grid[i][j] > max_height {
				visibles[i][j] = true;
				max_height = grid[i][j];
			}
		}
	}

	for j in 0..grid[0].len() {
		let mut max_height = 0;
		for i in 0..grid.len() {
			if grid[i][j] > max_height {
				visibles[i][j] = true;
				max_height = grid[i][j];
			}
		}

		max_height = 0;
		for i in (0..grid.len()).rev() {
			if grid[i][j] > max_height {
				visibles[i][j] = true;
				max_height = grid[i][j];
			}
		}
	}

	visibles.into_iter().flatten().filter(|v| *v).count()
}

fn get_max_views_score(grid: &Vec<Vec<u8>>) -> u32 {
	let mut max_score = 0;

	for i in 0..grid.len() {
		for j in 0..grid[0].len() {
			let (i, j) = (i as isize, j as isize);
			let l = count_clear_view(&grid, (i, j), (0, -1));
			let r = count_clear_view(&grid, (i, j), (0, 1));
			let t = count_clear_view(&grid, (i, j), (-1, 0));
			let b = count_clear_view(&grid, (i, j), (1, 0));
			let score = l * r * t * b;
			if score > max_score {
				max_score = score;
			}
		}
	}

	max_score
}

fn count_clear_view(grid: &Vec<Vec<u8>>, pos: (isize, isize), inc: (isize, isize)) -> u32 {
	let (mut i, mut j) = pos;
	let ref_height = grid[i as usize][j as usize];

	let i_range = 0..grid.len() as isize;
	let j_range = 0..grid[0].len() as isize;

	let mut count = 0;
	loop {
		i += inc.0;
		j += inc.1;
		let tree_exists = i_range.contains(&i) && j_range.contains(&j);

		if tree_exists {
			count += 1;
		}

		if !tree_exists || grid[i as usize][j as usize] >= ref_height {
			break;
		}
	}

	count
}
