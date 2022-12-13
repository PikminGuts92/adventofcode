#[cfg(test)] mod data;

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
        // delimited(tag("["), separated_list0(tag(","), tag(",")), tag("]"))
        alt((
            map_res(digit1, |s: &'a str| s.parse::<i32>().map(|d| PacketData::Number(d))),
            map_res(
                delimited(
                    tag("["),
                    separated_list0(tag(","), Self::parse_packet),
                    tag("]")
                ),
                |packets: Vec<PacketData>| -> Result<PacketData, Box<dyn std::error::Error>> { Ok(PacketData::Array(packets)) })
        ))(data)
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

pub fn count_packets_in_correct_order(packets: &[[PacketData; 2]]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 13)]
    #[case(TEST_DATA_1, 0)]
    fn count_packets_in_correct_order_test(#[case] raw_data: &str, #[case] expected: i32) {
        let packets = parse_packet_data(raw_data);
        let result = count_packets_in_correct_order(&packets);

        assert_eq!(expected, result);
    }
}