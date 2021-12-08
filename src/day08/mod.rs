#[cfg(test)] mod data;

use std::collections::HashMap;

pub const DIGIT_0: &'static str = "aaaabbcceeffgggg";
pub const DIGIT_1: &'static str = "ccff";
pub const DIGIT_2: &'static str = "aaaaccddddeegggg";
pub const DIGIT_3: &'static str = "aaaaccddddffgggg";
pub const DIGIT_4: &'static str = "bbccddddff";
pub const DIGIT_5: &'static str = "aaaabbddddffgggg";
pub const DIGIT_6: &'static str = "aaaabbddddeeffgggg";
pub const DIGIT_7: &'static str = "aaaaccff";
pub const DIGIT_8: &'static str = "aaaabbccddddeeffgggg";
pub const DIGIT_9: &'static str = "aaaabbccddddgggg";

/*pub fn parse_segment(segment: &str) -> ([&str; 10], [&str; 4]) {
    ()
}*/

pub fn char_count(digit: &'static str) -> i32 {
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

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1_PARSED, 26)]
    #[case(TEST_DATA_2_PARSED, 392)]
    pub fn segment_search_find_1478_test<T: AsRef<[([&'static str; 10], [&'static str; 4])]>>(#[case] data: T, #[case] expected: i32) {
        let result = segment_search_find_1478(data.as_ref());
        assert_eq!(expected, result);
    }
}