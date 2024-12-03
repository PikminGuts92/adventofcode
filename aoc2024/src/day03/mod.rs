use nom::*;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{is_not, tag, take, take_till, take_till1, take_while, take_while1, take_until};
use nom::character::{is_alphanumeric, is_digit, is_hex_digit};
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1, digit1, hex_digit1, newline, one_of, space0, space1};
use nom::combinator::{all_consuming, map, map_opt, map_parser, map_res, not, opt, peek, recognize, value};
use nom::error::{context, Error};
use nom::multi::{fold_many0, many0, separated_list0, separated_list1};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};

#[cfg(test)] mod data;

#[derive(Debug)]
pub enum Operation {
    Multiply(i32, i32)
}

impl Operation {
    pub fn resolve(&self) -> i32 {
        match self {
            Operation::Multiply(a, b) => a * b
        }
    }
}

fn parse_do_dont<'a>(text: &'a str) -> IResult<&'a str, bool> {
    map(
        alt((
            tag("do()"),
            tag("don't()")
        )),
        |d: &str| d.eq("do()")
    )(text)
}

fn parse_operation<'a>(text: &'a str) -> IResult<&'a str, Operation> {
    preceded(
        tag("mul"),
        delimited(
            tag("("),
            map(
                separated_pair(digit1, tag(","), digit1),
                |(d1, d2): (&str, &str)| Operation::Multiply(d1.parse::<i32>().unwrap(), d2.parse::<i32>().unwrap())
            ),
            tag(")"))
        )(text)
}

fn parse_operations(data: &[&str]) -> Vec<Operation> {
    let mut operations = Vec::new();

    for line in data {
        for i in 0..line.len() {
            let Ok((_, op)) = parse_operation(&line[i..]) else {
                continue;
            };

            operations.push(op);
        }
    }

    operations
}

fn parse_operations_with_do_dont(data: &[&str]) -> Vec<Operation> {
    let mut operations = Vec::new();
    let mut enabled = true;

    for line in data {
        for i in 0..line.len() {
            if let Ok((_, do_dont)) = parse_do_dont(&line[i..]) {
                enabled = do_dont;
                continue;
            };

            if !enabled {
                continue;
            }

            let Ok((_, op)) = parse_operation(&line[i..]) else {
                continue;
            };

            operations.push(op);
        }
    }

    operations
}

fn find_and_calculate_sum_of_products(data: &[&str], do_dont: bool) -> i32 {
    let operations = if do_dont {
        parse_operations_with_do_dont(data)
    } else {
        parse_operations(data)
    };

    operations
        .into_iter()
        .map(|op| op.resolve())
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 161)]
    #[case(TEST_DATA_1, 174960292)]
    fn find_and_calculate_sum_of_products_test<const N: usize>(#[case] data: [&str; N], #[case] expected: i32) {
        let actual = find_and_calculate_sum_of_products(&data, false);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_0_ALT, 48)]
    #[case(TEST_DATA_1, 56275602)]
    fn find_and_calculate_sum_of_products_with_do_dont_test<const N: usize>(#[case] data: [&str; N], #[case] expected: i32) {
        let actual = find_and_calculate_sum_of_products(&data, true);

        assert_eq!(expected, actual);
    }
}