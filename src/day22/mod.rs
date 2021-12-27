#[cfg(test)] mod data;

use std::collections::HashSet;
use std::ops::RangeInclusive;

pub fn find_on_cube_count(data: &[(bool, [RangeInclusive<i64>; 3])]) -> i64 {
    let mut occupied = HashSet::new();

    for (on, [x, y, z]) in data.iter() {
        for z_i in z.to_owned() {
            for y_i in y.to_owned() {
                for x_i in x.to_owned() {
                    if *on {
                        occupied.insert([x_i, y_i, z_i]);
                    } else {
                        occupied.remove(&[x_i, y_i, z_i]);
                    }
                }
            }
        }
    }

    occupied.len() as i64
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 39)]
    #[case(TEST_DATA_1, 590784)]
    #[case(TEST_DATA_2, 0)]
    pub fn find_on_cube_count_test<T: AsRef<[(bool, [RangeInclusive<i64>; 3])]>>(#[case] data: T, #[case] expected: i64) {
        let result = find_on_cube_count(data.as_ref());
        assert_eq!(expected, result);
    }
}