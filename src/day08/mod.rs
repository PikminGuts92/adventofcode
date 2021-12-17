#[cfg(test)] mod data;

use std::collections::HashMap;


pub struct Pattern<'a> {
    _digit: &'a str,
    char_count: i32,
    hash: u8,
}

impl<'a> Pattern<'a> {
    pub fn new(digit: &'a str) -> Pattern {
        let mut hash = 0u8;

        for c in digit.chars() {
            let idx = (c as u8) - ('a' as u8);
            hash |= 0b1 << idx;
        }

        let char_count = char_count(digit);

        Pattern {
            _digit: digit,
            char_count,
            hash
        }
    }

    pub fn get_chars(&self) -> Vec<char> {
        get_chars_from_hash(self.hash)
    }
}

pub fn get_chars_from_hash(hash: u8) -> Vec<char> {
    let mut chars = Vec::new();

    for (i, c) in ('a'..'h').enumerate() {
        if hash & (0b1 << i) != 0 {
            chars.push(c);
        }
    }

    chars
}

pub fn char_count(digit: &str) -> i32 {
    let mut counts = [0i32; 7];

    for c in digit.chars() {
        let idx = (c as u8) - ('a' as u8);
        counts[idx as usize] += 1;
    }

    counts
        .iter()
        .filter(|c| c > &&0)
        .count() as i32
}

pub fn char_count_from_hash(hash: u8) -> i32 {
    let mut count = 0;

    for i in 0..7 {
        if hash & (0b1 << i) != 0 {
            count += 1;
        }
    }

    count
}

pub fn is_one(char_count: i32) -> bool {
    char_count == 2
}

pub fn is_four(char_count: i32) -> bool {
    char_count == 4
}


pub fn is_seven(char_count: i32) -> bool {
    char_count == 3
}

pub fn is_eight(char_count: i32) -> bool {
    char_count == 7
}

pub fn segment_search_find_1478(data: &[([&'static str; 10], [&'static str; 4])]) -> i32 {
    let mut total_count = 0;

    for (_, digits) in data.iter() {
        for d in digits.iter() {
            let char_count = char_count(d);

            if is_one(char_count)
                || is_four(char_count)
                || is_seven(char_count)
                || is_eight(char_count) {
                    total_count += 1
                }
        }
    }

    total_count
}

pub fn segment_search_item((patterns, digits): &([&'static str; 10], [&'static str; 4])) -> i32 {
    //let mut segment_arrangement = [None; 7];
    let mut patterns_map =  [None; 10];

    // Init as 0, 1, 2, 3, etc.
    /*segment_arrangement
        .iter_mut()
        .enumerate()
        .for_each(|(i, sa)| *sa = i as u32);*/

    let mut char_counts = [0i32; 10];
    for (i, p) in patterns.iter().enumerate() {
        let count = char_count(p);
        char_counts[i] = count;

        if is_one(count) {
            patterns_map[1] = Some(i);
        } else if is_four(count) {
            patterns_map[4] = Some(i);
        } else if is_seven(count) {
            patterns_map[7] = Some(i);
        } else if is_eight(count) {
            patterns_map[8] = Some(i);
        }
    }

    let pattern_objs = patterns
        .map(|p| Pattern::new(p));

    // Unique segment counts (givens)
    let one_hash = pattern_objs[patterns_map[1].unwrap()].hash;
    let four_hash = pattern_objs[patterns_map[4].unwrap()].hash;
    let seven_hash = pattern_objs[patterns_map[7].unwrap()].hash;
    let eight_hash = pattern_objs[patterns_map[8].unwrap()].hash;

    /*let first_segment = one_hash ^ seven_hash;
    let first_segment_char = *get_chars_from_hash(first_segment).first().unwrap();

    println!("First char is {}", first_segment_char);*/

    // 5 segments
    let three_hash = pattern_objs
        .iter()
        .filter(|p| p.char_count == 5)
        .map(|p| p.hash)
        .reduce(|acc, h| acc & h)
        .unwrap() | one_hash;

    // 6 segments
    let nine_hash = three_hash | four_hash;
    let zero_hash = ((one_hash ^ four_hash) & three_hash) ^ eight_hash;
    let six_hash = pattern_objs
        .iter()
        .filter(|p| p.char_count == 6
            && p.hash != nine_hash
            && p.hash != zero_hash)
        .map(|p| p.hash)
        .nth(0)
        .unwrap();

    // 5 segments (remaining)
    let five_hash = pattern_objs
        .iter()
        .filter(|p| p.char_count == 5
            && p.hash != three_hash
            && char_count_from_hash(p.hash ^ six_hash) == 1)
        .map(|p| p.hash)
        .nth(0)
        .unwrap();
    let two_hash = pattern_objs
        .iter()
        .filter(|p| p.char_count == 5
            && p.hash != three_hash
            && p.hash != five_hash)
        .map(|p| p.hash)
        .nth(0)
        .unwrap();

    // Update patterns
    for (i, p) in pattern_objs.iter().enumerate() {
        let map_idx = match p.hash {
            h if h == zero_hash => 0,
            h if h == two_hash => 2,
            h if h == three_hash => 3,
            h if h == five_hash => 5,
            h if h == six_hash => 6,
            h if h == nine_hash => 9,
            _ => continue
        };

        patterns_map[map_idx] = Some(i);
    }

    // 5 chars
    // (2 & 3 & 5) | (1 || 7) == 3

    // (3 | 4) == 9

    // ((1 ^ 4) & 3) ^ 8 = 0

    // (1 | 4 | 7) ^ 9 == 'gggg' slot

    let hashes = [
        zero_hash,
        one_hash,
        two_hash,
        three_hash,
        four_hash,
        five_hash,
        six_hash,
        seven_hash,
        eight_hash,
        nine_hash
    ].iter()
        .enumerate()
        .map(|(i, e)| (*e, i as i32))
        .collect::<HashMap<_, _>>();

    // Figure out digits
    let mut number = 0;
    for (i, d) in digits.iter().rev().enumerate() {
        let hash = Pattern::new(d).hash;

        let digit = hashes[&hash];
        number += digit * (10i32.pow(i as u32));
    }

    number
}

pub fn segment_search(data: &[([&'static str; 10], [&'static str; 4])]) -> i32 {
    data
        .iter()
        .map(|d| segment_search_item(d))
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 26)]
    #[case(TEST_DATA_2, 392)]
    pub fn segment_search_find_1478_test<T: AsRef<[([&'static str; 10], [&'static str; 4])]>>(#[case] data: T, #[case] expected: i32) {
        let result = segment_search_find_1478(data.as_ref());
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_0, 0, 5353)]
    #[case(TEST_DATA_1, 0, 8394)]
    #[case(TEST_DATA_1, 1, 9781)]
    #[case(TEST_DATA_1, 2, 1197)]
    pub fn segment_search_item_test<T: AsRef<[([&'static str; 10], [&'static str; 4])]>>(#[case] data: T, #[case] i: usize, #[case] expected: i32) {
        let result = segment_search_item(&data.as_ref()[i]);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_1, 61229)]
    #[case(TEST_DATA_2, 1004688)]
    pub fn segment_search_test<T: AsRef<[([&'static str; 10], [&'static str; 4])]>>(#[case] data: T, #[case] expected: i32) {
        let result = segment_search(data.as_ref());
        assert_eq!(expected, result);
    }
}