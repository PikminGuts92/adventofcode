#[cfg(test)] mod data;

use std::collections::HashMap;

pub const DIGIT_0: &'static str = "aaaabbcceeffgggg";     //         6
pub const DIGIT_1: &'static str = "ccff";                 // 2
pub const DIGIT_2: &'static str = "aaaaccddddeegggg";     //       5
pub const DIGIT_3: &'static str = "aaaaccddddffgggg";     //       5
pub const DIGIT_4: &'static str = "bbccddddff";           //     4
pub const DIGIT_5: &'static str = "aaaabbddddffgggg";     //       5
pub const DIGIT_6: &'static str = "aaaabbddddeeffgggg";   //         6
pub const DIGIT_7: &'static str = "aaaaccff";             //   3
pub const DIGIT_8: &'static str = "aaaabbccddddeeffgggg"; //           7
pub const DIGIT_9: &'static str = "aaaabbccddddgggg";     //         6

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

    println!("Done");

    0
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
    #[case(TEST_DATA_1_PARSED, 26)]
    #[case(TEST_DATA_2_PARSED, 392)]
    pub fn segment_search_find_1478_test<T: AsRef<[([&'static str; 10], [&'static str; 4])]>>(#[case] data: T, #[case] expected: i32) {
        let result = segment_search_find_1478(data.as_ref());
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_0_PARSED, 0, 5353)]
    #[case(TEST_DATA_1_PARSED, 0, 8394)]
    #[case(TEST_DATA_1_PARSED, 1, 9781)]
    #[case(TEST_DATA_1_PARSED, 2, 1197)]
    pub fn segment_search_item_test<T: AsRef<[([&'static str; 10], [&'static str; 4])]>>(#[case] data: T, #[case] i: usize, #[case] expected: i32) {
        let result = segment_search_item(&data.as_ref()[i]);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_1_PARSED, 61229)]
    #[case(TEST_DATA_2_PARSED, 0)]
    pub fn segment_search_test<T: AsRef<[([&'static str; 10], [&'static str; 4])]>>(#[case] data: T, #[case] expected: i32) {
        let result = segment_search(data.as_ref());
        assert_eq!(expected, result);
    }
}