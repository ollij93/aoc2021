// Solutions for day16 of Advent of Code

use std::cmp::{max, min};

use super::common::run_and_print_time;

fn string_to_bytes(s: &str) -> Vec<bool> {
    s.chars()
        .flat_map(|c| match c {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => panic!("Non hex character in input: {}", c),
        })
        .map(|b| b == 1)
        .collect()
}

fn bits_as_u128(bits: &[bool]) -> u128 {
    bits.iter()
        .fold(0, |curr, bit| curr * 2 + if *bit { 1 } else { 0 })
}

enum PacketOperator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}
enum Packet {
    Literal {
        version: u8,
        value: u128,
    },
    Operator {
        version: u8,
        operator: PacketOperator,
        subpackets: Vec<Packet>,
    },
}
impl Packet {
    fn version_sum(&self) -> u32 {
        match self {
            Packet::Literal { version, value: _ } => *version as u32,
            Packet::Operator {
                version,
                operator: _,
                subpackets,
            } => *version as u32 + subpackets.iter().map(|p| p.version_sum()).sum::<u32>(),
        }
    }

    fn evaluate(&self) -> u128 {
        match self {
            Packet::Literal { version: _, value } => *value,
            Packet::Operator {
                version: _,
                operator,
                subpackets,
            } => match operator {
                PacketOperator::Sum => subpackets.iter().map(|p| p.evaluate()).sum::<u128>(),
                PacketOperator::Product => subpackets.iter().map(|p| p.evaluate()).product(),
                PacketOperator::Minimum => subpackets
                    .iter()
                    .map(|p| p.evaluate())
                    .fold(std::u128::MAX, min),
                PacketOperator::Maximum => subpackets.iter().map(|p| p.evaluate()).fold(0, max),
                PacketOperator::GreaterThan => {
                    if subpackets[0].evaluate() > subpackets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                PacketOperator::LessThan => {
                    if subpackets[0].evaluate() < subpackets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                PacketOperator::EqualTo => {
                    if subpackets[0].evaluate() == subpackets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

fn parse_value(input: &[bool]) -> (u128, usize) {
    let mut bits = input;
    let mut value = bits_as_u128(&bits[1..5]);
    let mut bitsread = 5;
    while bits[0] {
        bits = &bits[5..];
        value = value * 16 + bits_as_u128(&bits[1..5]);
        bitsread += 5;
    }
    (value, bitsread)
}

fn parse_subpackets_length(input: &[bool]) -> (Vec<Packet>, usize) {
    let mut bits = input;
    let length = bits_as_u128(&bits[0..15]) as usize;
    bits = &bits[15..];
    let mut totalread = 15;
    let mut packets = vec![];
    loop {
        let (packet, bitsread) = parse_packet(bits);
        totalread += bitsread;
        packets.push(packet);

        // Check if we're done
        if totalread - 15 >= length {
            break;
        }

        bits = &bits[bitsread..];
    }
    (packets, totalread)
}

fn parse_subpackets_number(input: &[bool]) -> (Vec<Packet>, usize) {
    let mut bits = input;
    let number = bits_as_u128(&bits[0..11]);
    bits = &bits[11..];
    let mut totalread = 11;
    let mut packets = vec![];
    for _ in 0..number {
        let (packet, bitsread) = parse_packet(bits);
        totalread += bitsread;
        packets.push(packet);
        bits = &bits[bitsread..];
    }
    (packets, totalread)
}

fn parse_packet(bits: &[bool]) -> (Packet, usize) {
    let version = bits_as_u128(&bits[0..3]) as u8;
    let typeid = bits_as_u128(&bits[3..6]);
    let operator = match typeid {
        0 => Some(PacketOperator::Sum),
        1 => Some(PacketOperator::Product),
        2 => Some(PacketOperator::Minimum),
        3 => Some(PacketOperator::Maximum),
        4 => None,
        5 => Some(PacketOperator::GreaterThan),
        6 => Some(PacketOperator::LessThan),
        7 => Some(PacketOperator::EqualTo),
        _ => panic!("Unsupported typeid: {}", typeid),
    };
    match operator {
        None => {
            let (value, bitsread) = parse_value(&bits[6..]);
            (Packet::Literal { version, value }, bitsread + 6)
        }
        Some(op) => {
            if bits[6] {
                let (subpackets, bitsread) = parse_subpackets_number(&bits[7..]);
                (
                    Packet::Operator {
                        version,
                        operator: op,
                        subpackets,
                    },
                    bitsread + 7,
                )
            } else {
                let (subpackets, bitsread) = parse_subpackets_length(&bits[7..]);
                (
                    Packet::Operator {
                        version,
                        operator: op,
                        subpackets,
                    },
                    bitsread + 7,
                )
            }
        }
    }
}

fn p1(input: &[String]) -> Vec<u32> {
    input
        .iter()
        .map(|line| string_to_bytes(line))
        .map(|bits| parse_packet(&bits))
        .map(|(packet, _)| packet.version_sum())
        .collect()
}

fn p2(input: &[String]) -> Vec<u128> {
    input
        .iter()
        .map(|line| string_to_bytes(line))
        .map(|bits| parse_packet(&bits))
        .map(|(packet, _)| packet.evaluate())
        .collect()
}

pub fn run(input: Vec<String>) -> u128 {
    println!("=== DAY 16 ===");

    let (a, timea) = run_and_print_time(p1, &input);
    for x in a {
        println!("Part1: {}", x);
    }

    let (b, timeb) = run_and_print_time(p2, &input);
    for x in b {
        println!("Part2: {}", x);
    }

    timea + timeb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            p1(&[
                "8A004A801A8002F478".to_string(),
                "620080001611562C8802118E34".to_string(),
                "C0015000016115A2E0802F182340".to_string(),
                "A0016C880162017C3686B18A3D4780".to_string()
            ]),
            vec![16, 12, 23, 31]
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            p2(&[
                "C200B40A82".to_string(),
                "04005AC33890".to_string(),
                "880086C3E88112".to_string(),
                "CE00C43D881120".to_string(),
                "D8005AC2A8F0".to_string(),
                "F600BC2D8F".to_string(),
                "9C005AC2F8F0".to_string(),
                "9C0141080250320F1802104A08".to_string()
            ]),
            vec![3, 54, 7, 9, 1, 0, 0, 1]
        )
    }
}
