#[cfg(test)] mod data;

use std::ops::Range;

pub fn find_highest_velocity(boundry: &[Range<i64>; 2], start: (i64, i64)) -> (i64, i64) {

    Default::default()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, (0, 0), 9)]
    pub fn find_highest_velocity_test(#[case] boundry: [Range<i64>; 2], #[case] start: (i64, i64), #[case] expected: i64) {
        let (_, result_y) = find_highest_velocity(&boundry, start);
        assert_eq!(expected, result_y);
    }
}