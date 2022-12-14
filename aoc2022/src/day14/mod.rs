#[cfg(test)] mod data;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while, take_while1};
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair, terminated};

type Coordinate = (usize, usize);

fn parse_coordinates<'a>(data: &'a str) -> IResult<&'a str, Vec<Coordinate>> {
    separated_list0(
        tag(" -> "),
        map(
            separated_pair(
                digit1,
                tag(","),
                digit1
            ),
            |(x, y): (&str, &str)| (
                x.parse::<usize>().unwrap(),
                y.parse::<usize>().unwrap()
            )
        )
    )(data)
}

pub fn parse_coordinate_data(data: &[&str]) -> Vec<Vec<Coordinate>> {
    data
        .iter()
        .map(|s| parse_coordinates(s).map(|(_, c)| c).unwrap())
        .collect()
}

pub fn simulate_falling_sand(sand_coordinates: &[Vec<Coordinate>]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 24)]
    #[case(TEST_DATA_1, 0)]
    fn count_packets_in_correct_order_test<const N: usize>(#[case] raw_data: [&str; N], #[case] expected: i32) {
        let coordinates = parse_coordinate_data(&raw_data);
        let result = simulate_falling_sand(&coordinates);

        assert_eq!(expected, result);
    }
}