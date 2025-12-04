#[cfg(test)] mod data;

fn find_max_joltage_for_bank(bank: &str) -> i64 {
    //let mut max_left = 0;
    //let mut max_right = 0;

    //let mut left_idx = 0;
    //let mut right_idx = 1;

    let mut max_joltage = 0;

    for (i, left_char) in bank.chars().enumerate() {
        for right_char in bank.chars().skip(i + 1) {
            /*if max_left == 9 && max_right == 9 {
                break;
            }*/

            /*if max_joltage == 99 {
                return max_joltage;
            }*/

            let left_num = left_char.to_digit(10).unwrap() as i64;
            let right_num = right_char.to_digit(10).unwrap() as i64;

            let joltage = (left_num * 10) + right_num;
            if joltage > max_joltage {
                max_joltage = joltage
            }
        }
    }

    //println!("Bank: {bank}, Joltage: {max_joltage}");
    max_joltage
}

fn calculate_max_joltage_sum(data: &[&str]) -> i64 {
    data
        .iter()
        .map(|d| find_max_joltage_for_bank(d))
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 357)]
    #[case(TEST_DATA_1, 17100)]
    fn calculate_max_joltage_sum_test<const N: usize>(#[case] data: [&str; N], #[case] expected: i64) {
        let actual = calculate_max_joltage_sum(&data);
        assert_eq!(expected, actual);
    }
}