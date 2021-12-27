#[cfg(test)] mod data;

use std::ops::RangeInclusive;

pub fn find_on_cube_count(data: &[(bool, [RangeInclusive<i64>; 3])]) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 39)]
    #[case(TEST_DATA_1, 590784)]
    #[case(TEST_DATA_2, 39)]
    pub fn find_on_cube_count_test<T: AsRef<[(bool, [RangeInclusive<i64>; 3])]>>(#[case] data: T, #[case] expected: i64) {
        let result = find_on_cube_count(data.as_ref());
        assert_eq!(expected, result);
    }
}