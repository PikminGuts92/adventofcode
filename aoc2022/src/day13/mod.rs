#[cfg(test)] mod data;

use itertools::{Itertools, EitherOrBoth};
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while, take_while1};
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::{delimited, terminated};
use std::cmp::Ordering;

#[derive(Debug)]
pub enum PacketData {
    Array(Vec<PacketData>),
    Number(i32)
}

impl PacketData {
    pub fn from_str<'a>(data: &'a str) -> PacketData {
        let (_, packet) = Self::parse_packet(data).unwrap();
        packet
    }

    fn parse_packet<'a>(data: &'a str) -> IResult<&'a str, PacketData> {
        alt((
            map_res(
                digit1,
                |s: &'a str| s.parse::<i32>().map(|d| PacketData::Number(d))),
            map_res(
                delimited(
                    tag("["),
                    separated_list0(tag(","), Self::parse_packet),
                    tag("]")
                ),
                |packets: Vec<PacketData>| -> Result<PacketData, Box<dyn std::error::Error>> {
                    Ok(PacketData::Array(packets))
                })
        ))(data)
    }

    pub fn to_string(&self) -> String {
        let mut working_string = String::new();
        self.append_to_string(&mut working_string);
        working_string
    }

    fn append_to_string(&self, working_string: &mut String) {
        match self {
            PacketData::Number(n) => working_string.push_str(&format!("{n}")),
            PacketData::Array(arr) => {
                working_string.push('[');

                for (i, p) in arr.iter().enumerate() {
                    p.append_to_string(working_string);

                    if (i + 1) < arr.len() {
                        working_string.push(',');
                    }
                }

                working_string.push(']');
            }
        }
    }
}

pub fn parse_packet_data(data: &str) -> Vec<[PacketData; 2]> {
    data
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>() // Needs to be collected for chunks?
        .chunks(2)
        .map(|c| [
            PacketData::from_str(c[0]),
            PacketData::from_str(c[1])
        ])
        .collect()
}

pub fn get_order(a: &PacketData, b: &PacketData) -> Ordering {
    match (a, b) {
        (PacketData::Number(na), PacketData::Number(nb)) => na.cmp(nb),
        (pa @ PacketData::Number(_), PacketData::Array(arrb)) => compare_arrays_in_order(
            vec![pa].into_iter(),
            arrb.iter()
        ),
        (PacketData::Array(arra), pb @ PacketData::Number(_)) => compare_arrays_in_order(
            arra.iter(),
            vec![pb].into_iter(),
        ),
        (PacketData::Array(arra), PacketData::Array(arrb)) => compare_arrays_in_order(
            arra.iter(),
            arrb.iter(),
        )
    }
}

pub fn compare_arrays_in_order<'a, T: Iterator<Item = &'a PacketData>, S: Iterator<Item = &'a PacketData>>(arra: T, arrb: S) -> Ordering {
    let orders = arra
        .into_iter()
        .zip_longest(arrb)
        .map(|res| match res {
            EitherOrBoth::Both(pa, pb) => get_order(pa, pb),
            EitherOrBoth::Right(_) => Ordering::Less, // Left side ran out of items
            _ => Ordering::Greater,
        });

    for order in orders {
        match order {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => {}
        }
    }

    Ordering::Equal
}

pub fn count_packets_in_correct_order(packets: &[[PacketData; 2]]) -> i32 {
    let mut correct_pairs = 0;

    for (i, [p1, p2]) in packets.iter().enumerate() {
        let result = match get_order(p1, p2) {
            Ordering::Greater | Ordering::Equal => false,
            _ => true,
        };

        if result {
            correct_pairs += i as i32 + 1;
        }
    }

    correct_pairs
}

pub fn sort_and_find_divider_packets(packets: &mut Vec<PacketData>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 13)]
    #[case(TEST_DATA_1, 5882)]
    fn count_packets_in_correct_order_test(#[case] raw_data: &str, #[case] expected: i32) {
        let packets = parse_packet_data(raw_data);
        let result = count_packets_in_correct_order(&packets);

        assert_eq!(expected, result);
    }
}