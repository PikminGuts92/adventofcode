use std::collections::HashMap;

#[cfg(test)] mod data;

fn calculate_sum_of_invalid_ids(data: &str) -> i64 {
    let ids = data
        .split_terminator(",")
        .map(|ids| ids.split_terminator("-").take(2).enumerate().fold(["", ""], |mut acc, (i, id)| {
            acc[i] = id;
            acc
        }))
        .collect::<Vec<_>>();

    let mut invalid_id_sum = 0;

    for [start_id_str, end_id_str] in ids {
        let start_id = start_id_str.parse::<i64>().unwrap();
        let end_id = end_id_str.parse::<i64>().unwrap();

        for id in start_id..=end_id {
            let is_valid = is_valid_number(id);
            if !is_valid {
                invalid_id_sum += id;
            }
        }
    }

    invalid_id_sum
}

fn is_valid_number(num: i64) -> bool {
    //let num_str = format!("{num}").chars().collect::<Vec<_>>();
    let num_str = format!("{num}");
    if is_odd(num_str.len() as i64) {
        return true;
    }

    let (n1, n2) = num_str.split_at(num_str.len() / 2);
    n1 != n2

    //let mut working_num_str = String::new();
    //true
}

fn is_odd(num: i64) -> bool {
    (num | 1) == 1
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 1227775554)]
    #[case(TEST_DATA_1, 28146997880)]
    fn calculate_sum_of_invalid_ids_test(#[case] data: &str, #[case] expected: i64) {
        let actual = calculate_sum_of_invalid_ids(data);
        assert_eq!(expected, actual);
    }
}