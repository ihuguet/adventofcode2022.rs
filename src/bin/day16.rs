use adventofcode2022 as aoc;
use std::collections::{HashSet, HashMap};
// use itertools::Itertools;

struct Valve {
	flow: i32,
	tunnels: Vec<usize>,
	distances: Vec<usize>
}

#[derive(Debug)]
struct State {
	id: usize,
	unvisited: HashSet<usize>,
	minutes: i32,
	pressure: i32,
}

fn main() {
	let (mut valves, id_start) = parse_input("day16");
	let ids_with_flow = precompute(&mut valves, id_start);

	println!("Part 1: max released pressure {}", part1(&valves, id_start, &ids_with_flow));
}

fn precompute(valves: &mut [Valve], id_start: usize) -> Vec<usize> {
	let ids_with_flow = valves.iter().enumerate()
		.filter(|(_i, v)| v.flow > 0)
		.map(|(i, _v)| i)
		.collect::<Vec<_>>();

	valves[id_start].distances = calc_distances_from(&valves, id_start);
	for &valve_id in &ids_with_flow {
		valves[valve_id].distances = calc_distances_from(valves, valve_id);
	} 

	ids_with_flow
}

fn calc_distances_from(valves: &[Valve], id: usize) -> Vec<usize> {
	let mut distances = vec![usize::MAX; valves.len()];
	let mut queue = vec![id];
	distances[id] = 0;

	while let Some(id) = queue.pop() {
		let dist = distances[id];

		for &neigh in &valves[id].tunnels {
			if distances[neigh] > dist + 1 {
				distances[neigh] = dist + 1;
				queue.push(neigh);
			}
		}
	}

	distances
}

fn part1(valves: &[Valve], id_start: usize, ids_with_flow: &[usize]) -> i32 {
	let mut max_pressure = 0;
	let mut queue = vec![State {
		id: id_start,
		unvisited: ids_with_flow.into_iter().copied().collect(),
		minutes: 30,
		pressure: 0 
	}];

	while let Some(state) = queue.pop() {
		let State {
			id: id_prev, unvisited: unvisited_prev, minutes: minutes_prev, pressure: pressure_prev
		} = state;

		for &id in &unvisited_prev {
			let dist = valves[id_prev].distances[id];
			let mut minutes = minutes_prev - dist as i32;
			let mut pressure = 0;

			if minutes > 0 {
				minutes -= 1;
				pressure = pressure_prev + minutes * valves[id].flow;
				if pressure > max_pressure {
					max_pressure = pressure;
				}
			}

			if minutes > 0 {
				let mut unvisited = unvisited_prev.clone();
				unvisited.remove(&id);

				if !unvisited.is_empty() { // && pressure + best_pressure > max_pressure {
					queue.push(State {id, minutes, unvisited, pressure});
				}
			}
		}
	}

	max_pressure
}

// fn best_potential_pressure(valves: &[Valve], unvisited: &HashSet<usize>, from: usize, minutes: i32)
// 	-> i32
// {
// 	unvisited.into_iter().map(|id| {
// 		valves[*id].flow * (minutes - 1 - valves[from].distances[*id] as i32)
// 	}).sum()
// }

fn parse_input(day_xx: &str) -> (Vec<Valve>, usize) {
	let mut ids_map = HashMap::new();
	let mut id_start = 0;

	let values = aoc::input::read_lines(day_xx)
		.into_iter()
		.enumerate()
		.map(|(id, line)| {
			let line_parts = line.split("; ").collect::<Vec<_>>();
			let valve = line_parts[0][6.. 8].to_string();
			let flow = line_parts[0][23..].parse().unwrap();
			let tunnels = line_parts[1][22..].split(", ").map(|s| s.trim().to_string()).collect::<Vec<_>>();

			if valve == "AA" {
				id_start = id;
			}
			ids_map.insert(valve, id);

			(flow, tunnels)
		}).collect::<Vec<_>>();
	
	let valves = values.into_iter()
		.map(|(flow, tunnels)| {
			Valve {
				flow,
				tunnels: tunnels.into_iter().map(|v| ids_map[&v]).collect(),
				distances: Vec::new()
			}
		}).collect();
	
	(valves, id_start)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part1() {
		let (mut valves, id_start) = parse_input("day16-test");
		let ids_with_flow = precompute(&mut valves, id_start);
		assert_eq!(part1(&valves, id_start, &ids_with_flow), 1651);
	}
}