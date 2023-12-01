use nom::*;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{is_not, tag, take, take_till, take_till1, take_while, take_while1};
use nom::character::{is_alphanumeric, is_digit, is_hex_digit};
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1, digit1, hex_digit1, one_of};
use nom::combinator::{all_consuming, map, map_opt, map_parser, map_res, not, opt, peek, recognize};
use nom::error::{context, Error};
use nom::multi::{fold_many0, many0, separated_list0};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};

// https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md

#[cfg(test)] mod data;

fn parse_digit<'a>(text: &'a [u8]) -> IResult<&'a [u8], u8> {
    alt((
        map(alt((tag(b"0"), tag(b"zero"))), |_| b'0'),
        map(alt((tag(b"1"), tag(b"one"))), |_| b'1'),
        map(alt((tag(b"2"), tag(b"two"))), |_| b'2'),
        map(alt((tag(b"3"), tag(b"three"))), |_| b'3'),
        map(alt((tag(b"4"), tag(b"four"))), |_| b'4'),
        map(alt((tag(b"5"), tag(b"five"))), |_| b'5'),
        map(alt((tag(b"6"), tag(b"six"))), |_| b'6'),
        map(alt((tag(b"7"), tag(b"seven"))), |_| b'7'),
        map(alt((tag(b"8"), tag(b"eight"))), |_| b'8'),
        map(alt((tag(b"9"), tag(b"nine"))), |_| b'9'),
    ))(text)
}

fn parse_digits<'a>(text: &'a [u8]) -> IResult<&'a [u8], Vec<u8>> {
    fold_many0(
        alt((
            terminated(peek(map(parse_digit, |d| Some(d))), opt(take(1u32))),
            map(take(1u32), |_| None),
        )),
        Vec::new,
        |mut acc: Vec<u8>, mut item| {
            if let Some(d) = item.take() {
                acc.push(d);
            }

            acc
        }
    )(text)
}

fn get_digits_from_row_1(row: &[u8]) -> u32 {
    let left_digit = row
        .iter()
        .find(|c| c.is_ascii_digit())
        .unwrap();

    let right_digit = row
        .iter()
        .rev()
        .find(|c| c.is_ascii_digit())
        .unwrap();

    let s = String::from_utf8(vec![ *left_digit, *right_digit ]).unwrap();

    s.parse::<u32>().unwrap()
}

fn calculate_sum_from_rows_1(rows: &[&[u8]]) -> u32 {
    rows
        .iter()
        .map(|r| get_digits_from_row_1(r))
        .sum()
}

fn get_digits_from_row_2(row: &[u8]) -> u32 {
    let (_, digits) = parse_digits(row).unwrap();

    let left_digit = digits
        .first()
        .unwrap();

    let right_digit = digits
        .last()
        .unwrap();

    let s = String::from_utf8(vec![ *left_digit, *right_digit ]).unwrap();

    s.parse::<u32>().unwrap()
}

fn calculate_sum_from_rows_2(rows: &[&[u8]]) -> u32 {
    rows
        .iter()
        .map(|r| get_digits_from_row_2(r))
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 142)]
    #[case(TEST_DATA_1, 54450)]
    fn calculate_sum_from_rows_1_test<const N: usize>(#[case] data: [&[u8]; N], #[case] expected: u32) {
        let actual = calculate_sum_from_rows_1(&data);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_1_ALT, 281)]
    #[case(TEST_DATA_1, 54265)]
    fn calculate_sum_from_rows_2_test<const N: usize>(#[case] data: [&[u8]; N], #[case] expected: u32) {
        let actual = calculate_sum_from_rows_2(&data);

        assert_eq!(expected, actual);
    }
}