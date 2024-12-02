use std::ops::RangeInclusive;

#[cfg(test)] mod data;

fn is_row_safe(row: &[i32], range: RangeInclusive<i32>) -> bool {
    let mut prev_is_increasing = None;

    for i in 0..(row.len() - 1) {
        let a = row[i];
        let b = row[i + 1];

        let diff = a - b;
        let is_increasing = diff < 0;

        let is_increasing = match diff {
            d if d.is_negative() => true,
            d if d.is_positive() => false,
            _ => return false
        };

        if !range.contains(&diff.abs()) || prev_is_increasing.is_some_and(|p| p != is_increasing) {
            return false;
        }

        prev_is_increasing = Some(is_increasing);
    }

    true
}

fn is_row_safe_if_removed(row: &[i32], removed_idx: usize, range: RangeInclusive<i32>) -> bool {
    let mut prev_is_increasing = None;

    for i in 0..(row.len() - 1) {
        if i == removed_idx {
            continue;
        }

        let next_idx = if (i + 1) == removed_idx {
            i + 2
        } else {
            i + 1
        };

        if next_idx >= row.len() {
            // Nothing left to compare
            break;
        }

        let a = row[i];
        let b = row[next_idx];

        let diff = a - b;
        let is_increasing = diff < 0;

        let is_increasing = match diff {
            d if d.is_negative() => true,
            d if d.is_positive() => false,
            _ => return false
        };

        if !range.contains(&diff.abs()) || prev_is_increasing.is_some_and(|p| p != is_increasing) {
            return false;
        }

        prev_is_increasing = Some(is_increasing);
    }

    true
}

fn get_safe_count(data: &[&[i32]]) -> i32 {
    data
        .iter()
        .filter(|r| is_row_safe(r, 1..=3))
        .count() as i32
}

fn get_safe_count_if_removed(data: &[&[i32]]) -> i32 {
    data
        .iter()
        .filter(|r| is_row_safe(r, 1..=3)
            || (0..r.len()).any(|i| is_row_safe_if_removed(r, i, 1..=3)))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 2)]
    #[case(TEST_DATA_1, 236)]
    fn get_safe_count_test<const N: usize>(#[case] data: [&[i32]; N], #[case] expected: i32) {
        let actual = get_safe_count(&data);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_0, 4)]
    #[case(TEST_DATA_1, 308)]
    fn get_safe_count_if_removed_test<const N: usize>(#[case] data: [&[i32]; N], #[case] expected: i32) {
        let actual = get_safe_count_if_removed(&data);

        assert_eq!(expected, actual);
    }
}