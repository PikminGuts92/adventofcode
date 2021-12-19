#[cfg(test)] mod data;

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    /*#[rstest]
    #[case(TEST_DATA_1, 4140)]
    pub fn find_magnitude_of_snailfish_test<T: AsRef<[&'static str]>>(#[case] data: T, #[case] expected: i64) {
        let result = find_magnitude_of_snailfish(data.as_ref());
        assert_eq!(expected, result);
    }*/
}