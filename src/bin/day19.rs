use adventofcode2022 as aoc;
use Material::*;

const MAX_TIME_PART1: i32 = 24;
const MAX_TIME_PART2: i32 = 32;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Material {
	Ore,
	Clay,
	Obsidian,
	Geode
}

struct Blueprint([[i32; 4]; 4]);  // i = robot type, j = qty of required material

#[derive(Debug, Clone)]
struct State {
	robots: [i32; 4],
	materials: [i32; 4],
	minutes: i32
}

fn main() {
	let blueprints = parse_input("day19");
	println!("Part 1: quality levels sum {}", part1(&blueprints));
	println!("Part 2: quality levels sum {}", part2(&blueprints));
}

fn part1(blueprints: &[Blueprint]) -> i32 {
	blueprints.iter().enumerate()
		.map(|(idx, blueprint)| (idx as i32 + 1) * solve(blueprint, MAX_TIME_PART1))
		.sum()
}

fn part2(blueprints: &[Blueprint]) -> i32 {
	blueprints[..3].iter()
		.map(|blueprint| solve(blueprint, MAX_TIME_PART2))
		.product()
}

fn solve(blueprint: &Blueprint, max_minutes: i32) -> i32 {
	let mut queue = vec![
		State {
			robots: [1, 0, 0, 0],
			materials: [0, 0, 0, 0],
			minutes: 0
		}
	];
	let mut max_geodes = 0;

	while let Some(state) = queue.pop() {
		if end_condition(&state, max_minutes, max_geodes) {
			continue;
		}

		let next_robots = blueprint.get_next_robots(&state.robots);
		let left_minutes = max_minutes - state.minutes;

		for next_robot in next_robots {
			let minutes_inc = blueprint.minutes_to_build(next_robot, &state).min(left_minutes);

			let mut state = state.clone();
			state.minutes += minutes_inc;
			state.collect_materials(minutes_inc);
			state.create_robot(next_robot, blueprint);

			if state.materials[Geode as usize] > max_geodes {
				max_geodes = state.materials[Geode as usize];
			}

			if state.minutes >= max_minutes {
				continue;
			}

			queue.push(state);
		}
	}
	
	max_geodes
}

fn end_condition(state: &State, max_time: i32, max_geodes: i32) -> bool {
	let mut very_optimistic_geodes = state.materials[Geode as usize];
	for min in 0..max_time - state.minutes {
		very_optimistic_geodes += state.robots[Geode as usize] + min
	}
	very_optimistic_geodes <= max_geodes
}

impl Blueprint {
	fn get_next_robots(&self, robots: &[i32; 4]) -> Vec<Material> {
		[Ore, Clay, Obsidian, Geode].into_iter()
			.filter(|material| self.can_build_robot(robots, *material))
			.filter(|robot_type| !self.has_enough_robots(robots, *robot_type))
			.collect()
	}

	fn can_build_robot(&self, robots: &[i32; 4], robot_type: Material) -> bool {
		for (req_material, qty) in self.0[robot_type as usize].iter().enumerate() {
			if *qty > 0 && robots[req_material] == 0 {
				return false;
			}
		}
		true
	}

	fn has_enough_robots(&self, robots: &[i32; 4], robot_type: Material) -> bool {
		if robot_type == Geode {
			return false;
		}

		let enough_count = self.0.iter()
			.map(|robot_recipe| robot_recipe[robot_type as usize])
			.max().unwrap();

		robots[robot_type as usize] >= enough_count
	}

	fn minutes_to_build(&self, robot: Material, state: &State) -> i32 {
		let robot_recipe = &self.0[robot as usize];
		[Ore, Clay, Obsidian, Geode].into_iter()
			.map(|material| self.minutes_to_collect(material, robot_recipe[material as usize], state))
			.max()
			.unwrap() + 1
	}

	fn minutes_to_collect(&self, material: Material, qty: i32, state: &State) -> i32 {
		let left = qty - state.materials[material as usize];
		let robot_num = state.robots[material as usize];
		if left <= 0 {
			0
		} else if left % robot_num == 0 {
			left / robot_num
		} else {
			left / robot_num + 1
		}
	}
}

impl State {
	fn collect_materials(&mut self, minutes_inc: i32) {
		for (idx, qty) in self.materials.iter_mut().enumerate() {
			*qty += minutes_inc * self.robots[idx];
		}
	}

	fn create_robot(&mut self, robot: Material, blueprint: &Blueprint) {
		self.robots[robot as usize] += 1;
		for (idx, qty) in self.materials.iter_mut().enumerate() {
			*qty -= blueprint.0[robot as usize][idx];
		}
	}
}

fn parse_input(day_xx: &str) -> Vec<Blueprint> {
	aoc::input::read_lines(day_xx)
		.map(|line| {
			let mut cost = [[0; 4]; 4];
			let words: Vec<&str> = line.split(" ").collect();
			cost[Ore as usize][Ore as usize] = words[6].parse().unwrap();
			cost[Clay as usize][Ore as usize] = words[12].parse().unwrap();
			cost[Obsidian as usize][Ore as usize] = words[18].parse().unwrap();
			cost[Obsidian as usize][Clay as usize] = words[21].parse().unwrap();
			cost[Geode as usize][Ore as usize] = words[27].parse().unwrap();
			cost[Geode as usize][Obsidian as usize] = words[30].parse().unwrap();
			Blueprint(cost)
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_one_part1() {
		let blueprints = parse_input("day19-test");
		assert_eq!(solve(&blueprints[0], MAX_TIME_PART1), 9);
	}

	#[test]
	fn test_part1() {
		let blueprints = parse_input("day19-test");
		assert_eq!(part1(&blueprints), 33);
	}

	#[test]
	fn test_one_part2() {
		let blueprints = parse_input("day19-test");
		assert_eq!(solve(&blueprints[0], MAX_TIME_PART2), 56);
	}
}