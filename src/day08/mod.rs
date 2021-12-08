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

pub fn segment_search(data: &[([&'static str; 10], [&'static str; 4])]) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1_PARSED, 0)]
    //#[case(TEST_DATA_2, 376, 352707)]
    pub fn segment_search_test<T: AsRef<[([&'static str; 10], [&'static str; 4])]>>(#[case] data: T, #[case] expected: i32) {
        let result = segment_search(data.as_ref());
        assert_eq!(expected, result);
    }
}