use adventofcode2022 as aoc;
use itertools::Itertools;
use std::collections::{HashSet, HashMap};

struct Valve {
	flow: i32,
	tunnels: Vec<usize>,
	distances: Vec<usize>
}

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
}

fn main() {
	let (mut valves, id_start) = parse_input("day16");
	let ids_with_flow = precompute(&mut valves, id_start);

	println!("Part 1: max released pressure {}", part1(&valves, id_start, &ids_with_flow));
	println!("Part 2: max released pressure {}", part2(&valves, id_start, &ids_with_flow));
}

fn precompute(valves: &mut [Valve], id_start: usize) -> HashSet<usize> {
	let ids_with_flow = valves.iter().enumerate()
		.filter(|(_i, v)| v.flow > 0)
		.map(|(i, _v)| i)
		.collect::<HashSet<_>>();

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

fn part1(valves: &[Valve], id_start: usize, ids_with_flow: &HashSet<usize>) -> i32 {
	let mut max_pressure = 0;
	let mut queue = vec![StatePart1 {
		id: id_start,
		unvisited: ids_with_flow.clone(),
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

fn part2(valves: &[Valve], id_start: usize, ids_with_flow: &HashSet<usize>) -> i32 {
	let mut max_pressure = 0;
	let mut queue = vec![StatePart2 {
		destinations: [(id_start, 0), (id_start, 0)],
		unvisited: ids_with_flow.clone(),
		minutes: 27,  // +1 minute to compensate the first `minutes -= 1`
		pressure: 0,
	}];

	while let Some(state) = queue.pop() {
		let StatePart2 { destinations, unvisited, mut minutes, mut pressure} = state; 
		let (id_dest0, mut dist0) = destinations[0];
		let (id_dest1, mut dist1) = destinations[1];
		let open_valve0 = dist0 == 0;
		let open_valve1 = dist1 == 0;

		minutes -= 1;
		match open_valve0 {
			true  => pressure += valves[id_dest0].flow * minutes,
			false => dist0 -= 1,  // distance while the other opens his valve
		}
		match open_valve1 {
			true  => if id_dest0 != id_dest1 || !open_valve0 {  // avoid opening valve at the same time
				pressure += valves[id_dest1].flow * minutes
			},
			false => dist1 -= 1,  // idem
		}
		if pressure > max_pressure {
			max_pressure = pressure;
		}

		let is_last = unvisited.is_empty() && id_dest0 == id_dest1;
		let best_pressure = pressure + optimistic_potential_pressure_part2(valves, &unvisited, (id_dest0, open_valve0), (id_dest1, open_valve1), minutes);
		if is_last || best_pressure <= max_pressure {
			continue;
		}

		let next_dests = match (open_valve0, open_valve1) {
			(true, true) => {
				unvisited.iter()
					.permutations(2)
					.map(|ids| (*ids[0], *ids[1]))
					.collect::<Vec<_>>()
			},
			(true, false) if !unvisited.is_empty() => {
				std::iter::zip(
					unvisited.iter().copied(),
					std::iter::repeat(id_dest1)
				).collect::<Vec<_>>()
			},
			(false, true) if !unvisited.is_empty() => {
				std::iter::zip(
					std::iter::repeat(id_dest0),
					unvisited.iter().copied()
				).collect::<Vec<_>>()
			},
			(true, false) => {
				vec![(id_dest1, id_dest1)]
			},
			(false, true) => {
				vec![(id_dest0, id_dest0)]
			},
			(false, false) => {
				panic!("Error in the program: at least one should open a valve");
			},
		};

		for (id0_next, id1_next) in next_dests {
			let mut dist0_next = match open_valve0 {
				true  => valves[id_dest0].distances[id0_next],
				false => dist0,
			};
			let mut dist1_next = match open_valve1 {
				true  => valves[id_dest1].distances[id1_next],
				false => dist1,
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
			});
		}
	}

	max_pressure
}

fn optimistic_potential_pressure_part2(valves: &[Valve], unvisited: &HashSet<usize>,
	(id_dest0, open_valve0): (usize, bool), (id_dest1, open_valve1): (usize, bool), minutes: i32)
	-> i32
{
	let mut unvisited = unvisited.clone();
	if !open_valve0 {  // if not openned, it's also missing
		unvisited.insert(id_dest0);
	}
	if !open_valve1 {  // idem
		unvisited.insert(id_dest1);
	}

	unvisited.iter()
		.filter_map(|id| {
			let from0 = valves[*id].flow * (minutes - 1 - valves[id_dest0].distances[*id] as i32);
			let from1 = valves[*id].flow * (minutes - 1 - valves[id_dest1].distances[*id] as i32);
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