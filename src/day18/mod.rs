#[cfg(test)] mod data;

use std::collections::HashMap;
use std::str::Chars;

#[derive(Debug)]
pub enum SnailFish {
    Literal(i64),
    Pair(Box<SnailFish>, Box<SnailFish>),
}

impl SnailFish {
    pub fn from_str(data: &str) -> SnailFish {
        let mut chars = data.chars();
        parse_from_str(&mut chars)
    }
    
    pub fn calc_magnitude(&self) -> i64 {
        match self {
            SnailFish::Literal(v) => *v,
            SnailFish::Pair(a, b)
                => (a.calc_magnitude() * 3) + (b.calc_magnitude() * 2)
        }
    }

    pub fn is_pair(&self) -> bool {
        match self {
            SnailFish::Literal(_) => false,
            SnailFish::Pair(_, _) => true,
        }
    }
}

fn parse_from_str(chars: &mut Chars) -> SnailFish {
    parse_fish(chars, &mut 0)
}

fn parse_fish(chars: &mut Chars, char_index: &mut i64) -> SnailFish {
    let c = chars.next().unwrap();
    *char_index += 1;

    match c {
        '0'..='9' => {
            // Parse literal
            let digit = parse_number(chars, c.to_digit(10).unwrap() as i64, char_index);
            SnailFish::Literal(digit)
        },
        ',' | ']' => {
            // Just skip characters
            parse_fish(chars, char_index)
        }
        '[' => {
            // Parse pair
            let a = parse_fish(chars, char_index);
            let b = parse_fish(chars, char_index);

            SnailFish::Pair(Box::new(a), Box::new(b))
        },
        _ => unimplemented!("Unknown character: {c} at index {char_index}")
    }
}

fn parse_number(chars: &mut Chars, mut num: i64, char_index: &mut i64) -> i64 {
    while let Some(c) = chars.next() {
        *char_index += 1;

        if !c.is_digit(10) {
            break;
        }

        let digit = c.to_digit(10).unwrap() as i64;
        num = (num * 10) + digit;
    }

    num
}

pub fn find_magnitude_of_snailfish(data: &[&'static str]) -> i64 {
    // "Addition"

    // Reduce rules:
    //  If any pair is nested inside four pairs, the leftmost such pair explodes
    //  If any regular number is 10 or greater, the leftmost such regular number splits

    // Explode action:
    //  Re-distributes literal values to left/right pairs
    //  * Explode always occur before split *

    // Magnitude:
    //    Pair - (left * 3) + (right * 2)
    // Literal - Just its value

    // Parse raw data

    0
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 4140)]
    pub fn find_magnitude_of_snailfish_test<T: AsRef<[&'static str]>>(#[case] data: T, #[case] expected: i64) {
        let result = find_magnitude_of_snailfish(data.as_ref());
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case("[6,8]")]
    #[case("[5,[7,2]]")]
    #[case("[[6,5],9]")]
    #[case("[[4,5],[2,6]]")]
    #[case(TEST_DATA_0_0[0])]
    #[case(TEST_DATA_0_1[0])]
    #[case(TEST_DATA_1[0])]
    #[case(TEST_DATA_2[0])]
    #[case(TEST_DATA_2[3])]
    #[case(TEST_DATA_2[4])]
    pub fn parse_snailfish_test(#[case] data: &'static str) {
        let _fish = SnailFish::from_str(data);
    }

    #[rstest]
    #[case("[[1,2],[[3,4],5]]", 143)]
    #[case("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)]
    #[case("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)]
    #[case("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)]
    #[case("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)]
    #[case("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)]
    pub fn snailfish_magnitude_test(#[case] data: &'static str, #[case] expected: i64) {
        let fish = SnailFish::from_str(data);
        let result = fish.calc_magnitude();
        assert_eq!(expected, result);
    }
}