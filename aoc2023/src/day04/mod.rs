#[cfg(test)] mod data;

use nom::*;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{is_not, tag, take, take_till, take_till1, take_while, take_while1};
use nom::character::{is_alphanumeric, is_digit, is_hex_digit, is_space};
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1, digit1, hex_digit1, one_of, space1};
use nom::combinator::{all_consuming, map, map_opt, map_parser, map_res, not, opt, peek, recognize, value};
use nom::error::{context, Error};
use nom::multi::{fold_many0, many0, separated_list0, separated_list1};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use std::collections::HashSet;

fn parse_card_id<'a>(text: &'a str) -> IResult<&'a str, u32> {
    delimited(
        terminated(tag("Card"), space1),
        map(digit1, |nums: &str| nums.parse::<u32>().unwrap()),
        tag(":")
    )(text)
}

fn parse_single_numbers<'a>(text: &'a str) -> IResult<&'a str, Vec<u32>> {
    separated_list0(
        space1,
        map(digit1, |nums: &str| nums.parse::<u32>().unwrap()),
    )(text)
}

fn parse_double_numbers<'a>(text: &'a str) -> IResult<&'a str, (Vec<u32>, Vec<u32>)> {
    separated_pair(
        preceded(space1, parse_single_numbers),
        preceded(space1, tag("|")),
        preceded(space1, parse_single_numbers),
    )(text)
}

fn parse_card_row<'a>(text: &'a str) -> (u32, Vec<u32>, Vec<u32>) {
    let (nums, card_id) = parse_card_id(text).unwrap();
    let (_, (winners, played)) = parse_double_numbers(nums).unwrap();

    /*println!("Card id: {card_id}");
    println!("Winning: {winners:?}");
    println!("Played: {played:?}\n");*/

    (card_id, winners, played)
}

fn calculate_winning_numbers_score(data: &[&str]) -> u32 {
    let mut current_sum = 0;

    for data_row in data {
        let (id, winning_nums, played_nums) = parse_card_row(data_row);

        let winners = winning_nums
            .into_iter()
            .collect::<HashSet<_>>();

        let matches = played_nums
            .iter()
            .filter(|n| winners.contains(n))
            .count();

        if matches > 0 {
            current_sum += 1 << (matches - 1);
        }
    }

    current_sum
}

fn calculate_winning_numbers_2_score(data: &[&str]) -> u32 {
    let mut scores = vec![1u32; data.len()];

    // Calculate scores
    for (i, data_row) in data.into_iter().enumerate() {
        let (id, winning_nums, played_nums) = parse_card_row(data_row);

        let winners = winning_nums
            .into_iter()
            .collect::<HashSet<_>>();

        let matches = played_nums
            .iter()
            .filter(|n| winners.contains(n))
            .count();

        let points = scores[i];

        for j in (i + 1)..(i + 1 + matches) {
            if let Some(s) = scores.get_mut(j) {
                *s += points;
            }
        }
    }

    scores
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 13)]
    #[case(TEST_DATA_1, 18619)]
    fn calculate_winning_numbers_score_test<const N: usize>(#[case] data: [&str; N], #[case] expected: u32) {
        let actual = calculate_winning_numbers_score(&data);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_0, 30)]
    #[case(TEST_DATA_1, 8063216)]
    fn calculate_winning_numbers_score_2_test<const N: usize>(#[case] data: [&str; N], #[case] expected: u32) {
        let actual = calculate_winning_numbers_2_score(&data);

        assert_eq!(expected, actual);
    }
}