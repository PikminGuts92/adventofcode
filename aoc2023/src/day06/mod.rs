#[cfg(test)] mod data;

fn find_win_count(max_time: u64, record_distance: u64) -> u64 {
    let mut win_count = 0;

    for wait_time in 1..max_time {
        let rem_time = max_time - wait_time;
        let distance = wait_time * rem_time;

        if distance > record_distance {
            win_count += 1;
        }
    }

    win_count
}

fn find_product_of_win_counts(data: &[(u64, u64)]) -> u64 {
    let mut current_product = 1;

    for (max_time, record_distance) in data {
        let win_count = find_win_count(*max_time, *record_distance);
        current_product *= win_count;
    }

    current_product
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 288)]
    #[case(TEST_DATA_1, 1731600)]
    #[case(TEST_DATA_2, 71503)]
    #[case(TEST_DATA_3, 40087680)]
    fn find_product_of_win_counts_test<const N: usize>(#[case] data: [(u64, u64); N], #[case] expected: u64) {
        let actual = find_product_of_win_counts(&data);

        assert_eq!(expected, actual);
    }
}