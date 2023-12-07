#[cfg(test)] mod data;

fn find_product_of_win_counts(data: &[(u32, u32)]) -> u32 {
    let mut current_product = 1;

    for (max_time, record_distance) in data {
        let mut win_count = 0;

        for wait_time in 1..*max_time {
            let rem_time = max_time - wait_time;
            let distance = wait_time * rem_time;

            //println!("Distance: {distance}");

            if distance > *record_distance {
                win_count += 1;
            }
        }

        current_product *= win_count;

        //println!("Wins: {win_count}\n\n");
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
    fn dummy_test<const N: usize>(#[case] data: [(u32, u32); N], #[case] expected: u32) {
        let actual = find_product_of_win_counts(&data);

        assert_eq!(expected, actual);
    }
}