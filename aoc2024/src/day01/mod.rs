use std::collections::HashMap;

#[cfg(test)] mod data;

fn calculate_sum_of_small_pairs(data: &[(i32, i32)]) -> i32 {
    let mut nums_1 = data
        .iter()
        .map(|(x, _)| *x)
        .collect::<Vec<_>>();

    let mut nums_2 = data
        .iter()
        .map(|(_, y)| *y)
        .collect::<Vec<_>>();

    nums_1.sort();
    nums_2.sort();

    let mut sum = 0;

    for (n1, n2) in nums_1.iter().zip(&nums_2) {
        let diff = (n1 - n2).abs();
        sum += diff;
    }

    sum
}

fn calculate_sum_product_of_frequencies(data: &[(i32, i32)]) -> i32 {
    let nums_1 = data
        .iter()
        .map(|(x, _)| *x)
        .collect::<Vec<_>>();

    let nums_2_freq = data
        .iter()
        .map(|(_, y)| *y)
        .fold(HashMap::new(), |mut acc, x| {
            acc
                .entry(x)
                .and_modify(|e| { *e += 1 })
                .or_insert(1);

            acc
        });


    let mut sum = 0;

    for n1 in nums_1.iter() {
        let freq = nums_2_freq.get(n1).map(|x| *x).unwrap_or_default();
        sum += n1 * freq;
    }

    sum
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 11)]
    #[case(TEST_DATA_1, 2769675)]
    fn calculate_sum_of_small_pairs_test<const N: usize>(#[case] data: [(i32, i32); N], #[case] expected: i32) {
        let actual = calculate_sum_of_small_pairs(&data);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_0, 31)]
    #[case(TEST_DATA_1, 24643097)]
    fn calculate_sum_product_of_frequencies_test<const N: usize>(#[case] data: [(i32, i32); N], #[case] expected: i32) {
        let actual = calculate_sum_product_of_frequencies(&data);

        assert_eq!(expected, actual);
    }
}