use nom::*;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{is_not, tag, take, take_till, take_till1, take_while, take_while1};
use nom::character::{is_alphanumeric, is_digit, is_hex_digit};
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1, digit1, hex_digit1, one_of};
use nom::combinator::{all_consuming, map, map_opt, map_parser, map_res, not, opt, peek, recognize, value};
use nom::error::{context, Error};
use nom::multi::{fold_many0, many0, separated_list0};
use nom::number::complete::recognize_float;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};

#[cfg(test)] mod data;

fn parse_game_id<'a>(text: &'a str) -> IResult<&'a str, u32> {
    delimited(
        tag("Game "),
        map(digit1, |nums: &str| nums.parse::<u32>().unwrap()),
        tag(": ")
    )(text)
}

fn parse_dice_hand<'a>(text: &'a str) -> IResult<&'a str, Vec<(u32, usize)>> {
    separated_list0(
        tag(", "),
        separated_pair(
            map(digit1, |nums: &str| nums.parse::<u32>().unwrap()),
            tag(" "),
            alt((
                value(0, tag("red")),
                value(1, tag("green")),
                value(2, tag("blue"))
            ))
        )
    )(text)
}

fn parse_dice_hands<'a>(text: &'a str) -> IResult<&'a str, Vec<Vec<(u32, usize)>>> {
    separated_list0(
        tag("; "),
        parse_dice_hand
    )(text)
}

fn parse_game_input(game: &str) -> (u32, [u32; 3]) {
    let (dice_hands, id) = parse_game_id(game).unwrap();
    let (_, hands) = parse_dice_hands(dice_hands).unwrap();

    let mut max_dies = [0u32; 3];

    for hand in hands {
        for (count, idx) in hand {
            max_dies[idx] = max_dies[idx].max(count);
        }
    }

    (id, max_dies)
}

fn find_sum_of_possibilities(games: &[&str], [r1, g1, b1]: &[u32; 3]) -> u32 {
    let mut possible_count = 0u32;

    for game in games {
        let (id, [r2, g2, b2]) = parse_game_input(game);

        if *r1 >= r2 && *g1 >= g2 && *b1 >= b2 {
            possible_count += id;
        }
    }

    possible_count
}

fn find_sum_of_min_powers(games: &[&str]) -> u32 {
    let mut power_sum = 0u32;

    for game in games {
        let (_, [r, g, b]) = parse_game_input(game);

        power_sum += r * g * b;
    }

    power_sum
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, [12, 13, 14], 8)]
    #[case(TEST_DATA_1, [12, 13, 14], 2348)]
    fn find_sum_of_possibilities_test<const N: usize>(#[case] data: [&str; N], #[case] config: [u32; 3], #[case] expected: u32) {
        let actual = find_sum_of_possibilities(&data, &config);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_0, 2286)]
    #[case(TEST_DATA_1, 76008)]
    fn find_sum_of_min_powers_test<const N: usize>(#[case] data: [&str; N], #[case] expected: u32) {
        let actual = find_sum_of_min_powers(&data);

        assert_eq!(expected, actual);
    }
}