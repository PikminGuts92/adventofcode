#[cfg(test)] mod data;

pub fn find_first_non_repeating_index(data: &str, count: usize) -> usize {
    if data.len() <= count {
        return 0;
    }

    let mut current_idx = count;
    let mut current_counts = [0u8; 26];
    let mut current_set = vec!['\0'; count].into_boxed_slice();

    for (i, c) in data.chars().take(count).enumerate() {
        current_set[i] = c;
        current_counts[c as usize - 'a' as usize] += 1;
    }

    for c in data.chars().skip(count) {
        // Check current counts
        let is_unique = current_counts
            .iter()
            .filter(|c| c.eq(&&1))
            .count() == count;

        if is_unique {
            break;
        }

        // Continue checking
        let dropped_char = current_set[0];
        current_counts[dropped_char as usize - 'a' as usize] -= 1;

        current_set.rotate_left(1);
        current_set[count - 1] = c;
        current_counts[c as usize - 'a' as usize] += 1;

        current_idx += 1;
    }

    current_idx
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 7)]
    #[case(TEST_DATA_1, 5)]
    #[case(TEST_DATA_2, 6)]
    #[case(TEST_DATA_3, 10)]
    #[case(TEST_DATA_4, 11)]
    #[case(TEST_DATA_5, 0)]
    fn find_first_non_repeating_index_test(#[case] data: &str, #[case] expected: usize) {
        let result = find_first_non_repeating_index(data, 4);

        assert_eq!(expected, result);
    }
}