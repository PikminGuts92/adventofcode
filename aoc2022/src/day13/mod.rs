#[cfg(test)] mod data;

use itertools::{Itertools, EitherOrBoth};
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while, take_while1};
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::{delimited, terminated};

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

pub fn is_in_order(a: &PacketData, b: &PacketData) -> bool {
    match (a, b) {
        (PacketData::Number(na), PacketData::Number(nb)) => na.le(nb),
        (pa @ PacketData::Number(_), PacketData::Array(arrb)) => compare_arrays_in_order(
            vec![pa].into_iter(),
            arrb.iter()
        ),
        (PacketData::Array(arra), pb @ PacketData::Number(_)) => compare_arrays_in_order(
            arra.iter().take(1),
            vec![pb].into_iter(),
        ),
        (PacketData::Array(arra), PacketData::Array(arrb)) => compare_arrays_in_order(
            arra.iter(),
            arrb.iter(),
        )
    }
}

pub fn compare_arrays_in_order<'a, T: Iterator<Item = &'a PacketData>, S: Iterator<Item = &'a PacketData>>(arra: T, arrb: S) -> bool {
    arra
        .into_iter()
        .zip_longest(arrb)
        .inspect(|res| {
            //println!("{res:?}");
        })
        .map(|res| match res {
            EitherOrBoth::Both(pa, pb) => is_in_order(pa, pb),
            EitherOrBoth::Right(_) => true, // Left side ran out of items
            _ => false,
        })
        .inspect(|is_in_order| {
            //println!("\t{} in order", if *is_in_order { "is" } else { "is not" });
        })
        .all(|r| r)
}

pub fn count_packets_in_correct_order(packets: &[[PacketData; 2]]) -> i32 {
    let mut correct_pairs = 0;

    for (i, [p1, p2]) in packets.iter().enumerate() {
        // DEBUG
        //println!("{p1:#?}");
        //println!("{p2:#?}");

        println!("{}", p1.to_string());
        println!("{}", p2.to_string());
        println!();

        continue;

        let result = is_in_order(p1, p2);
        if result {
            correct_pairs += i as i32 + 1;
            println!("Pair {}: In Order", i + 1);
        } else {
            println!("Pair {}: OUT OF ORDER", i + 1)
        }
    }

    correct_pairs
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 13)]
    #[case(TEST_DATA_1, 533)] // 533 too low
    fn count_packets_in_correct_order_test(#[case] raw_data: &str, #[case] expected: i32) {
        let packets = parse_packet_data(raw_data);
        let result = count_packets_in_correct_order(&packets);

        assert_eq!(expected, result);
    }
}