#[cfg(test)] mod data;

use std::collections::HashMap;
use std::fmt;
use std::ops::Add;
use std::str::Chars;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn get_id() -> usize { COUNTER.fetch_add(1, Ordering::Relaxed) }

pub enum ReduceAction {
    Explode(usize, i64, i64),
    ExplodeL(usize, i64),
    ExplodeR(usize, i64)
}

impl ReduceAction {
    pub fn with_id(mut self, new_id: usize) -> ReduceAction {
        let id = match &mut self {
            ReduceAction::Explode(id, _, _)
                | ReduceAction::ExplodeL(id, _)
                | ReduceAction::ExplodeR(id, _) => id,
        };

        *id = new_id;
        self
    }
}

//#[derive(Debug)]
pub enum SnailFish {
    Literal(usize, i64),
    Pair(usize, Box<SnailFish>, Box<SnailFish>),
}

impl SnailFish {
    pub fn from_str(data: &str) -> SnailFish {
        let mut chars = data.chars();
        parse_from_str(&mut chars)
    }

    pub fn id(&self) -> usize {
        match self {
            SnailFish::Literal(id, _) => *id,
            SnailFish::Pair(id, _, _) => *id,
        }
    }

    pub fn calc_magnitude(&self) -> i64 {
        match self {
            SnailFish::Literal(_, n) => *n,
            SnailFish::Pair(_, a, b)
                => (a.calc_magnitude() * 3) + (b.calc_magnitude() * 2)
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            SnailFish::Literal(_, _) => true,
            _ => false
        }
    }

    pub fn is_pair(&self) -> bool {
        !self.is_literal()
    }

    pub fn is_pair_with_literals(&self) -> bool {
        match self {
            SnailFish::Pair(_, a, b)
                => a.is_literal() && b.is_literal(),
            _ => false,
        }
    }

    pub fn reduce_root(&mut self) -> Option<ReduceAction> {
        self.reduce(0)
    }

    pub fn distribute_to_first_literal(&mut self, amount: i64, orig_id: usize) -> bool {
        if self.id().eq(&orig_id) {
            return false;
        }

        match self {
            SnailFish::Literal(_, n) => {
                *n += amount;
                true
            }
            SnailFish::Pair(_, a, b) => {
                a.distribute_to_first_literal(amount, orig_id) || b.distribute_to_first_literal(amount, orig_id)
            }
        }
    }

    pub fn reduce(&mut self, depth: i64) -> Option<ReduceAction> {
        if self.is_reduced(depth) {
            return None;
        }

        // First split, then explode

        match self {
            SnailFish::Literal(id, n) => {
                if *n >= 10 {
                    // Split into pair
                    let l = *n / 2;
                    let r = *n - l;

                    *self = SnailFish::Pair(
                        *id,
                        Box::new(SnailFish::Literal(get_id(), l)),
                        Box::new(SnailFish::Literal(get_id(), r)),
                    );

                    // Check if should be reduced again
                    return self.reduce(depth);
                }
            },
            SnailFish::Pair(id, a, b) => {
                while !a.is_reduced(depth + 1) || !b.is_reduced(depth + 1) {
                    // Reduce left
                    while let Some(a_action) = a.reduce(depth + 1) {
                        if depth.ge(&4) {
                            // Propagate action up
                            return Some(a_action.with_id(*id));
                        }

                        match a_action {
                            ReduceAction::Explode(action_id, l, r) => {
                                // Attempt distributing numbers
                                let a_consumed = a.distribute_to_first_literal(l, action_id);
                                let b_consumed = b.distribute_to_first_literal(r, action_id);

                                let new_action = match (a_consumed, b_consumed) {
                                    (false, false) => Some(ReduceAction::Explode(*id, l, r)),
                                    (false, true) => Some(ReduceAction::ExplodeL(*id, l)),
                                    (true, false) => Some(ReduceAction::ExplodeR(*id, r)),
                                    _ => None
                                };

                                // Propagate action up
                                if new_action.is_some() && depth > 0 {
                                    return new_action;
                                }
                            }
                            ReduceAction::ExplodeL(action_id, l) => {
                                let a_consumed = a.distribute_to_first_literal(l, action_id);

                                if !a_consumed && depth > 0 {
                                    // Propagate action up
                                    return Some(a_action.with_id(*id));
                                }
                            }
                            ReduceAction::ExplodeR(action_id, r) => {
                                let b_consumed = b.distribute_to_first_literal(r, action_id);

                                if !b_consumed && depth > 0 {
                                    // Propagate action up
                                    return Some(a_action.with_id(*id));
                                }
                            }
                        }
                    }

                    // Reduce right
                    while let Some(b_action) = b.reduce(depth + 1) {
                        if depth.ge(&4) {
                            // Propagate action up
                            return Some(b_action.with_id(*id));
                        }

                        match b_action {
                            ReduceAction::Explode(action_id, l, r) => {
                                // Attempt distributing numbers
                                let a_consumed = a.distribute_to_first_literal(l, action_id);
                                let b_consumed = b.distribute_to_first_literal(r, action_id);

                                let new_action = match (a_consumed, b_consumed) {
                                    (false, false) => Some(ReduceAction::Explode(*id, l, r)),
                                    (false, true) => Some(ReduceAction::ExplodeL(*id, l)),
                                    (true, false) => Some(ReduceAction::ExplodeR(*id, r)),
                                    _ => None
                                };

                                // Propagate action up
                                if new_action.is_some() && depth > 0 {
                                    return new_action;
                                }
                            }
                            ReduceAction::ExplodeL(action_id, l) => {
                                let a_consumed = a.distribute_to_first_literal(l, action_id);

                                if !a_consumed && depth > 0 {
                                    // Propagate action up
                                    return Some(b_action.with_id(*id));
                                }
                            }
                            ReduceAction::ExplodeR(action_id, r) => {
                                let b_consumed = b.distribute_to_first_literal(r, action_id);

                                if !b_consumed && depth > 0 {
                                    // Propagate action up
                                    return Some(b_action.with_id(*id));
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
                        (SnailFish::Literal(_, a), SnailFish::Literal(_, b)) => Some(ReduceAction::Explode(*id,*a, *b)),
                        _ => None,
                    };

                    // Propagate action up
                    if action.is_some() {
                        // Ugh... hacky debug code
                        if let Some(ReduceAction::Explode(id, _, _)) = action {
                            let new_self = SnailFish::Pair(
                                id,
                                Box::new(SnailFish::Literal(a.id(), a.calc_magnitude())),
                                Box::new(SnailFish::Literal(b.id(), b.calc_magnitude())),
                            );
                            println!("Exploding: {new_self} w/ id {}", new_self.id());
                        }

                        *self = SnailFish::Literal(*id, 0);

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
            SnailFish::Literal(_, n) => n.lt(&10),
            SnailFish::Pair(_, a, b)
                => depth.lt(&4) && a.is_reduced(depth + 1) && b.is_reduced(depth + 1),
        }
    }
}

impl Add for SnailFish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair(get_id(), Box::new(self), Box::new(rhs))
    }
}

impl fmt::Display for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnailFish::Literal(_, n) => write!(f, "{n}"),
            SnailFish::Pair(_, a, b) => write!(f, "[{a},{b}]")
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
            SnailFish::Literal(get_id(), digit)
        },
        ',' | ']' => {
            // Just skip characters
            parse_fish(chars, char_index)
        }
        '[' => {
            // Parse pair
            let a = parse_fish(chars, char_index);
            let b = parse_fish(chars, char_index);

            SnailFish::Pair(get_id(), Box::new(a), Box::new(b))
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

pub fn snailfish_add_set(data: &[&'static str]) -> SnailFish {
    // Parse raw data
    let fish = data
        .iter()
        .map(|d| SnailFish::from_str(d))
        .collect::<Vec<_>>();

    // Perform reduction + summazation
    let mut fish_sum = fish
        .into_iter()
        .reduce(|mut sum, mut fish| {
            println!("{sum}");

            sum.reduce_root();
            fish.reduce_root();

            sum + fish
        }).unwrap();

    println!("{fish_sum}");

    fish_sum.reduce_root();

    println!("{fish_sum}");
    fish_sum
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

    // Perform reduction + summazation
    let mut fish_sum = fish
        .into_iter()
        .reduce(|mut sum, mut fish| {
            sum.reduce_root();
            fish.reduce_root();

            sum + fish
        }).unwrap();

    fish_sum.reduce_root();
    println!("{fish_sum}");

    fish_sum.calc_magnitude()
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
    #[case("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")]
    pub fn snailfish_add_reduce_test(#[case] data_1: &'static str, #[case] data_2: &'static str, #[case] expected: &'static str) {
        let fish_1 = SnailFish::from_str(data_1);
        let fish_2 = SnailFish::from_str(data_2);

        let mut fish = fish_1 + fish_2;
        fish.reduce_root();

        assert_eq!(expected, format!("{fish}"));
    }

    #[rstest]
    #[case(TEST_DATA_ADD_0, "[[[[1,1],[2,2]],[3,3]],[4,4]]")]
    #[case(TEST_DATA_ADD_1, "[[[[3,0],[5,3]],[4,4]],[5,5]]")]
    #[case(TEST_DATA_ADD_2, "[[[[5,0],[7,4]],[5,5]],[6,6]]")]
    pub fn snailfish_add_set_test<T: AsRef<[&'static str]>>(#[case] data: T, #[case] expected: &'static str) {
        let fish = snailfish_add_set(data.as_ref());
        assert_eq!(expected, format!("{fish}"));
    }
}