use std::collections::HashSet;

#[cfg(test)] mod data;

fn calculate_correct_order_count(order_data: &[(i32, i32)], update_data: &[&[i32]]) -> i32 {
    let order_set = order_data
        .into_iter()
        .collect::<HashSet<_>>();

    let mut sum = 0;

    'update: for update in update_data {
        for i in 0..(update.len() - 1) {
            for j in (i + 1)..update.len() {
                let a = update[i];
                let b = update[j];

                if order_set.contains(&(b, a)) {
                    continue 'update
                }
            }
        }

        let middle_value = update[update.len() / 2];
        sum += middle_value;
    }

    sum
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0_0, TEST_DATA_0_1, 143)]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 4135)]
    fn calculate_correct_order_count_test<const N: usize, const M: usize>(#[case] order_data: [(i32, i32); N], #[case] update_data: [&[i32]; M], #[case] expected: i32) {
        let actual = calculate_correct_order_count(&order_data, &update_data);

        assert_eq!(expected, actual);
    }
}