#[cfg(test)] mod data;

fn get_digits_from_row_1(row: &str) -> u32 {
    let left_digit = row
        .chars()
        .find(|c| c.is_numeric())
        .unwrap();

    let right_digit = row
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .unwrap();

    let s = String::from_utf8(vec![ left_digit as u8, right_digit as u8 ]).unwrap();

    s.parse::<u32>().unwrap()
}

fn calculate_sum_from_rows_1(rows: &[&str]) -> u32 {
    rows
        .iter()
        .map(|r| get_digits_from_row_1(r))
        .sum()
}

fn get_digits_from_row_2(row: &str) -> u32 {
    let left_digit = row
        .chars()
        .find(|c| c.is_numeric())
        .unwrap();

    let right_digit = row
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .unwrap();

    let s = String::from_utf8(vec![ left_digit as u8, right_digit as u8 ]).unwrap();

    s.parse::<u32>().unwrap()
}

fn calculate_sum_from_rows_2(rows: &[&str]) -> u32 {
    rows
        .iter()
        .map(|r| get_digits_from_row_2(r))
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 142)]
    #[case(TEST_DATA_1, 54450)]
    fn calculate_sum_from_rows_1_test<const N: usize>(#[case] data: [&str; N], #[case] expected: u32) {
        let actual = calculate_sum_from_rows_1(&data);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_2, 0)]
    fn calculate_sum_from_rows_2_test<const N: usize>(#[case] data: [&str; N], #[case] expected: u32) {
        let actual = calculate_sum_from_rows_2(&data);

        assert_eq!(expected, actual);
    }
}