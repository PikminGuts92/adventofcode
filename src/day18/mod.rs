#[cfg(test)] mod data;

use std::collections::HashMap;
use std::fmt;
use std::ops::Add;
use std::str::Chars;

pub enum ReduceAction {
    Explode(i64, i64, i64),
    ExplodeL(i64, i64),
    ExplodeR(i64, i64)
}

//#[derive(Debug)]
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
            SnailFish::Literal(n) => *n,
            SnailFish::Pair(a, b)
                => (a.calc_magnitude() * 3) + (b.calc_magnitude() * 2)
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            SnailFish::Literal(_) => true,
            _ => false
        }
    }

    pub fn is_pair(&self) -> bool {
        !self.is_literal()
    }

    pub fn is_pair_with_literals(&self) -> bool {
        match self {
            SnailFish::Pair(a, b)
                => a.is_literal() && b.is_literal(),
            _ => false,
        }
    }

    pub fn reduce_root(&mut self) -> Option<ReduceAction> {
        self.reduce(0)
    }

    pub fn distribute_left(&mut self, amount: i64) -> bool {
        match self {
            SnailFish::Literal(n) => {
                *n += amount;
                true
            }
            SnailFish::Pair(a, _) => a.distribute_left(amount)
        }
    }

    pub fn distribute_right(&mut self, amount: i64) -> bool {
        match self {
            SnailFish::Literal(n) => {
                *n += amount;
                true
            }
            SnailFish::Pair(a, _) => a.distribute_right(amount)
        }
    }

    pub fn reduce(&mut self, depth: i64) -> Option<ReduceAction> {
        if self.is_reduced(depth) {
            return None;
        }

        // First split, then explode

        match self {
            SnailFish::Literal(n) => {
                if *n >= 10 {
                    // Split into pair
                    let l = *n / 2;
                    let r = *n - l;

                    *self = SnailFish::Pair(
                        Box::new(SnailFish::Literal(l)),
                        Box::new(SnailFish::Literal(r)),
                    );

                    // Check if should be reduced again
                    return self.reduce(depth);
                }
            },
            SnailFish::Pair(a, b) => {
                while !a.is_reduced(depth + 1) || !b.is_reduced(depth + 1) {
                    // Reduce left
                    while let Some(a_action) = a.reduce(depth + 1) {
                        if depth.ge(&4) {
                            // Propagate action up
                            return Some(a_action);
                        }

                        match a_action {
                            ReduceAction::Explode(d, l, r) => {
                                let mut a_consumed = false;
                                let mut b_consumed = false;

                                if d.ne(&(depth + 1)) && a.is_literal() {
                                    // Consume action and update a
                                    *a = Box::new(SnailFish::Literal(l + a.calc_magnitude()));
                                    a_consumed = true;
                                }

                                if b.is_literal() {
                                    // Consume action and update b
                                    *b = Box::new(SnailFish::Literal(r + b.calc_magnitude()));
                                    b_consumed = true;
                                }

                                let new_action = match (a_consumed, b_consumed) {
                                    (false, false) => Some(ReduceAction::Explode(d, l, r)),
                                    (false, true) => Some(ReduceAction::ExplodeL(d, l)),
                                    (true, false) => Some(ReduceAction::ExplodeR(d, r)),
                                    _ => None
                                };

                                // Propagate action up
                                if new_action.is_some() {
                                    return new_action;
                                }
                            }
                            ReduceAction::ExplodeL(d, l) => {
                                if a.is_literal() {
                                    // Consume action and update a
                                    *a = Box::new(SnailFish::Literal(l + a.calc_magnitude()));
                                } else if depth > 0 {
                                    // Propagate action up
                                    return Some(a_action);
                                }
                            }
                            ReduceAction::ExplodeR(d, r) => {
                                if b.is_literal() {
                                    // Consume action and update b
                                    *b = Box::new(SnailFish::Literal(r + b.calc_magnitude()));
                                } else {
                                    // Distribute to nested literal
                                    b.distribute_right(r);
                                }
                            }
                        }
                    }

                    // Reduce right
                    while let Some(b_action) = b.reduce(depth + 1) {
                        if depth.ge(&4) {
                            // Propagate action up
                            return Some(b_action);
                        }

                        match b_action {
                            ReduceAction::Explode(d, l, r) => {
                                let mut a_consumed = false;
                                let mut b_consumed = false;

                                if a.is_literal() {
                                    // Consume action and update a
                                    *a = Box::new(SnailFish::Literal(l + a.calc_magnitude()));
                                    a_consumed = true;
                                }

                                if d.ne(&(depth + 1)) && b.is_literal() {
                                    // Consume action and update b
                                    *b = Box::new(SnailFish::Literal(r + b.calc_magnitude()));
                                    b_consumed = true;
                                }

                                let new_action = match (a_consumed, b_consumed) {
                                    (false, false) => Some(ReduceAction::Explode(d, l, r)),
                                    (false, true) => Some(ReduceAction::ExplodeL(d, l)),
                                    (true, false) => Some(ReduceAction::ExplodeR(d, r)),
                                    _ => None
                                };

                                // Propagate action up
                                if new_action.is_some() {
                                    return new_action;
                                }
                            }
                            ReduceAction::ExplodeL(d, l) => {
                                if a.is_literal() {
                                    // Consume action and update a
                                    *a = Box::new(SnailFish::Literal(l + a.calc_magnitude()));
                                } else {
                                    // Distribute to nested literal
                                    a.distribute_right(l);
                                }
                            }
                            ReduceAction::ExplodeR(d, r) => {
                                if b.is_literal() {
                                    // Consume action and update b
                                    *b = Box::new(SnailFish::Literal(r + b.calc_magnitude()));
                                } else if depth > 0 {
                                    // Propagate action up
                                    return Some(b_action);
                                }
                            }
                        }
                    }
                }

                if depth.ge(&4) {
                    // Check left
                    /*while !a.is_reduced(depth + 1) {
                        match a.reduce(depth + 1) {
                            Some(ReduceAction::Explode(l, r)) =>
                        }
                    }*/

                    // Check right

                    // Explode pair if two literals

                    let action = match (a.as_ref(), b.as_ref()) {
                        (SnailFish::Literal(a), SnailFish::Literal(b)) => Some(ReduceAction::Explode(depth ,*a, *b)),
                        _ => None,
                    };

                    // Propagate action up
                    if action.is_some() {
                        *self = SnailFish::Literal(0);

                        return action;
                    }
                }

                // Check if should be reduced again
                //self.reduce(depth)?;
            }
        }

        None
    }

    pub fn is_reduced(&self, depth: i64) -> bool {
        match self {
            SnailFish::Literal(n) => n.lt(&10),
            SnailFish::Pair(a, b)
                => depth.lt(&4) && a.is_reduced(depth + 1) && b.is_reduced(depth + 1),
        }
    }
}

impl Add for SnailFish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair(Box::new(self), Box::new(rhs))
    }
}

impl fmt::Display for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailFish::Literal(n) => write!(f, "{n}"),
            SnailFish::Pair(a, b) => write!(f, "[{a},{b}]")
        }
    }
}

impl fmt::Debug for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
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
    let fish = data
        .iter()
        .map(|d| SnailFish::from_str(d))
        .collect::<Vec<_>>();

    for f in fish.iter() {
        println!("{f}");
    }

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

    #[rstest]
    #[case("[[[[[9,8],1],2],3],4]", 0, "[[[[0,9],2],3],4]")]
    #[case("[7,[6,[5,[4,[3,2]]]]]", 0, "[7,[6,[5,[7,0]]]]")]
    #[case("[[6,[5,[4,[3,2]]]],1]", 0, "[[6,[5,[7,0]]],3]")]
    #[case("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", 0, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
    #[case("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", 0, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
    pub fn snailfish_reduce_test(#[case] data: &'static str, #[case] depth: i64, #[case] expected: &'static str) {
        let mut fish = SnailFish::from_str(data);
        fish.reduce(depth);

        assert_eq!(expected, format!("{fish}"));
    }

    #[rstest]
    #[case("[1,2]", "3", "[[1,2],3]")]
    #[case("5", "3", "[5,3]")]
    #[case("[1,2]", "[3,4]", "[[1,2],[3,4]]")]
    #[case("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]", "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")]
    pub fn snailfish_add_test(#[case] data_1: &'static str, #[case] data_2: &'static str, #[case] expected: &'static str) {
        let fish_1 = SnailFish::from_str(data_1);
        let fish_2 = SnailFish::from_str(data_2);

        let fish = fish_1 + fish_2;

        assert_eq!(expected, format!("{fish}"));
    }

    #[rstest]
    #[case("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")]
    pub fn snailfish_add_reduce_test(#[case] data_1: &'static str, #[case] data_2: &'static str, #[case] expected: &'static str) {
        let fish_1 = SnailFish::from_str(data_1);
        let fish_2 = SnailFish::from_str(data_2);

        let mut fish = fish_1 + fish_2;
        fish.reduce_root();

        assert_eq!(expected, format!("{fish}"));
    }
}