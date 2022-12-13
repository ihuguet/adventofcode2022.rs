use adventofcode2022 as aoc;
use std::cmp::Ordering;

type Packet = Vec<Elem>;

#[derive(Debug, PartialEq, Clone)]
enum Elem {
	List(Packet),
	Num(u32),
}

fn main() {
	let packets = parse_input();
	println!("Part 1: sum of indices {}", part1(&packets));
	println!("Part 2: decoder key {}", part2(packets));
}

fn part1(packets: &[Packet]) -> u32 {
	packets.chunks(2).enumerate()
		.filter(|(_, pair)| packets_cmp(&pair[0], &pair[1]) == Ordering::Less)
		.map(|(i, _)| i as u32 + 1)
		.sum()
}

fn part2(mut packets: Vec<Packet>) -> u32 {
	let sep_2 = parse_packet("[[2]]").0;
	let sep_6 = parse_packet("[[6]]").0;

	packets.extend([sep_2.clone(), sep_6.clone()]);
	packets.sort_by(|a, b| packets_cmp(a, b));
	let sep_2_idx = 1 + packets.iter().position(|pkt| *pkt == sep_2).unwrap() as u32;
	let sep_6_idx = 1 + packets.iter().position(|pkt| *pkt == sep_6).unwrap() as u32;

	sep_2_idx * sep_6_idx
}

fn packets_cmp(lpacket: &[Elem], rpacket: &[Elem]) -> Ordering {
	for elem_pair in std::iter::zip(lpacket, rpacket) {
		let ord = match elem_pair {
			(Elem::Num(lnum), Elem::Num(rnum)) => lnum.cmp(&rnum),
			(Elem::List(llist), Elem::List(rlist)) => packets_cmp(&llist, &rlist),
			(Elem::List(llist), Elem::Num(rnum)) => packets_cmp(&llist, &[Elem::Num(*rnum)]),
			(Elem::Num(lnum), Elem::List(rlist)) => packets_cmp(&[Elem::Num(*lnum)], &rlist),
		};
		match ord {
			Ordering::Less | Ordering::Greater => return ord,
			Ordering::Equal => continue,
		}
	}

	lpacket.len().cmp(&rpacket.len())
}

fn parse_input() -> Vec<Packet> {
	aoc::input::read_lines("day13")
		.filter(|l| !l.is_empty())
		.map(|l| parse_packet(&l).0)
		.collect()
}

fn parse_packet(packet_str: &str) -> (Packet, usize) {
	let mut packet = Vec::new();
	let mut idx = 1;

	while idx < packet_str.len() {
		let (elem, end) = match packet_str.as_bytes()[idx] {
			b'[' => {
				let (packet, size) = parse_packet(&packet_str[idx..]);
				(Elem::List(packet), idx + size)
			},
			b'0'..=b'9' => {
				let end = idx + packet_str[idx..].find(&[',', ']']).unwrap();
				(Elem::Num(packet_str[idx..end].parse().unwrap()), end)
			},
			b']' if packet.len() == 0 => {  // empty packet is valid
				return (packet, idx + 1);
			}
			ch => {
				panic!("Unexpected char '{}', expected [ or 0-9 (idx: {}, packet: {})",
				       ch as char, idx, packet_str);
			},
		};

		packet.push(elem);

		match packet_str.as_bytes()[end] {
			b',' => idx = end + 1,
			b']' => return (packet, end + 1),
			ch  => panic!("Unexpected char '{}' after parsed element (idx: {}, packet: {})",
			              ch as char, end, packet_str),
		}
	}
	
	panic!("Unexpected end of packet (packet '{}')", packet_str);
}

#[cfg(test)]
mod test {
	use super::*;

	fn n(num: u32) -> Elem {
		Elem::Num(num)
	}

	fn l(list: Vec<Elem>) -> Elem {
		Elem::List(list)
	}

	#[test]
	fn test_parse_packet_ints() {
		let packet_str = "[1,1,3,1,1]";
		let expect = vec![n(1), n(1), n(3), n(1), n(1)];
		assert_eq!(parse_packet(packet_str).0, expect);
	}

	#[test]
	fn test_parse_packet_mix() {
		let packet_str = "[[4,4],4,4]";
		let expect = vec![l(vec![n(4), n(4)]), n(4), n(4)];
		assert_eq!(parse_packet(packet_str).0, expect);
	}

	#[test]
	fn test_parse_packet_empty() {
		let packet_str = "[[]]";
		let expect = vec![l(Vec::new())];
		assert_eq!(parse_packet(packet_str).0, expect);
	}
}