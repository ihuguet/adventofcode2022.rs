use adventofcode2022 as aoc;
use itertools::Itertools;
use std::collections::{HashSet, HashMap};

struct Valve {
	flow: i32,
	tunnels: Vec<usize>,
	distances: Vec<usize>
}

#[derive(Debug)]
struct StatePart1 {
	id: usize,
	unvisited: HashSet<usize>,
	minutes: i32,
	pressure: i32,
}

struct StatePart2 {
	destinations: [(usize, usize); 2], // (id, dist)
	unvisited: HashSet<usize>,
	minutes: i32,
	pressure: i32, 
	paths: (Vec<usize>, Vec<usize>)
}

fn main() {
	let (mut valves, id_start) = parse_input("day16");
	let ids_with_flow = precompute(&mut valves, id_start);

	println!("Part 1: max released pressure {}", part1(&valves, id_start, &ids_with_flow));
	println!("Part 2: max released pressure {}", part2(&valves, id_start, &ids_with_flow));
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
	let mut queue = vec![StatePart1 {
		id: id_start,
		unvisited: ids_with_flow.into_iter().copied().collect(),
		minutes: 30,
		pressure: 0 
	}];

	while let Some(state) = queue.pop() {
		let StatePart1 {
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

				let best_pressure = pressure +
					best_potential_pressure_part1(valves, &unvisited, id, minutes);

				if !unvisited.is_empty() && pressure + best_pressure > max_pressure {
					queue.push(StatePart1 {id, minutes, unvisited, pressure});
				}
			}
		}
	}

	max_pressure
}

fn best_potential_pressure_part1(valves: &[Valve], unvisited: &HashSet<usize>, from: usize, minutes: i32)
	-> i32
{
	unvisited.iter()
		.map(|id| valves[*id].flow * (minutes - 1 - valves[from].distances[*id] as i32))
		.filter(|v| *v > 0)
		.sum()
}

fn part2(valves: &[Valve], id_start: usize, ids_with_flow: &[usize]) -> i32 {
	let mut max_pressure = 0;
	let mut queue = vec![StatePart2 {
		destinations: [(id_start, 0), (id_start, 0)],
		unvisited: ids_with_flow.into_iter().copied().collect(),
		minutes: 27,  // +1 minute to compensate the first `minutes -= 1`
		pressure: 0,
		paths: (vec![], vec![]) 
	}];

	while let Some(state) = queue.pop() {
		let StatePart2 { destinations, unvisited, mut minutes, mut pressure, mut paths } = state;
		let (id0, dist0) = destinations[0];
		let (id1, dist1) = destinations[1];

		minutes -= 1;
		if dist0 == 0 {
			pressure += valves[id0].flow * minutes;
			paths.0.push(id0);
		}
		if dist1 == 0 {
			pressure += valves[id1].flow * minutes;
			paths.1.push(id1);
		}
		if pressure > max_pressure {
			max_pressure = pressure;
		}

		let best_pressure = pressure +
			best_potential_pressure_part2(valves, &unvisited, (id0, dist0), (id1, dist1), minutes);
		if best_pressure <= max_pressure {
			continue;
		}

		let next_dests = match (dist0, dist1) {
			(0, 0) => {
				unvisited.iter()
					.permutations(2)
					.map(|ids| (*ids[0], *ids[1]))
					.collect::<Vec<_>>()
			},
			(0, _) => {
				std::iter::zip(
					unvisited.iter().copied(),
					std::iter::repeat(id1)
				).collect::<Vec<_>>()
			},
			(_, 0) => {
				std::iter::zip(
					std::iter::repeat(id0),
					unvisited.iter().copied()
				).collect::<Vec<_>>()
			},
			_ => panic!("Error in the program, one of the dists must be 0")
		};

		// last valve to visit
		if next_dests.is_empty() && (dist0 != 0 || dist1 != 0) {
			let dist0_next = match dist0 {
				0 => valves[id0].distances[id1],
				d => d - 1 // -1 is distance during this last minute
			};
			let dist1_next = match dist1 {
				0 => valves[id1].distances[id0],
				d => d - 1 // -1 is distance during this last minute
			};
			let id_next = match dist0 {
				0 => id1,
				_ => id0,
			};

			let minutes_next = minutes - dist0_next.min(dist1_next) as i32 - 1;
			if minutes_next >= 0 {
				let pressure_next = pressure + valves[id_next].flow * minutes_next;
				if pressure_next > max_pressure {
					max_pressure = pressure_next;
					assert_eq!(unvisited.is_empty(), true);
				}
			}
			
			continue;
		}

		for (id0_next, id1_next) in next_dests {
			let mut dist0_next = match id0 == id0_next {
				true  => dist0 - 1,  // -1 is distance during this last minute
				false => valves[id0].distances[id0_next]
			};
			let mut dist1_next = match id1 == id1_next {
				true  => dist1 - 1,  // -1 is distance during this last minute
				false => valves[id1].distances[id1_next]
			};

			let min_dist = dist0_next.min(dist1_next);
			let minutes_next = minutes - min_dist as i32;
			dist0_next -= min_dist;
			dist1_next -= min_dist;

			if minutes_next <= 0 {
				continue;
			}

			let mut unvisited_next = unvisited.clone();
			unvisited_next.remove(&id0_next);
			unvisited_next.remove(&id1_next);

			queue.push(StatePart2 {
				destinations: [(id0_next, dist0_next), (id1_next, dist1_next)],
				unvisited: unvisited_next,
				minutes: minutes_next,
				pressure,
				paths: paths.clone()
			});
		}
	}

	max_pressure
}

fn best_potential_pressure_part2(valves: &[Valve], unvisited: &HashSet<usize>,
	dest0: (usize, usize), dest1: (usize, usize), minutes: i32)
	-> i32
{
	let mut unvisited = unvisited.clone();
	if dest0.1 != 0 {
		unvisited.insert(dest0.0);
	}
	if dest1.1 != 0 {
		unvisited.insert(dest1.0);
	}

	unvisited.iter()
		.filter_map(|id| {
			let from0 = valves[*id].flow * (minutes - 1 - valves[dest0.0].distances[*id] as i32);
			let from1 = valves[*id].flow * (minutes - 1 - valves[dest1.0].distances[*id] as i32);
			let pressure = from0.max(from1);
			match pressure {
				0.. => Some(pressure),
				_   => None
			}
		})
		.sum()
}

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

	#[test]
	fn test_part2() {
		let (mut valves, id_start) = parse_input("day16-test");
		let ids_with_flow = precompute(&mut valves, id_start);
		assert_eq!(part2(&valves, id_start, &ids_with_flow), 1707);
	}
}