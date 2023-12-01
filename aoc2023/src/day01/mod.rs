#[cfg(test)] mod data;

const DIGITS_AS_WORDS: [&[u8]; 10] = [
    b"zero",
    b"one",
    b"two",
    b"three",
    b"four",
    b"five",
    b"six",
    b"seven",
    b"eight",
    b"nine",
];

fn get_digits_from_row_1(row: &[u8]) -> u32 {
    let left_digit = row
        .iter()
        .find(|c| c.is_ascii_digit())
        .unwrap();

    let right_digit = row
        .iter()
        .rev()
        .find(|c| c.is_ascii_digit())
        .unwrap();

    let s = String::from_utf8(vec![ *left_digit, *right_digit ]).unwrap();

    s.parse::<u32>().unwrap()
}

fn calculate_sum_from_rows_1(rows: &[&[u8]]) -> u32 {
    rows
        .iter()
        .map(|r| get_digits_from_row_1(r))
        .sum()
}

fn get_digits_from_row_2(row: &[u8]) -> u32 {
    let left_digit = row
        .iter()
        .find(|c| c.is_ascii_digit())
        .unwrap();

    let right_digit = row
        .iter()
        .rev()
        .find(|c| c.is_ascii_digit())
        .unwrap();

    let s = String::from_utf8(vec![ *left_digit, *right_digit ]).unwrap();

    s.parse::<u32>().unwrap()
}

fn calculate_sum_from_rows_2(rows: &[&[u8]]) -> u32 {
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
    fn calculate_sum_from_rows_1_test<const N: usize>(#[case] data: [&[u8]; N], #[case] expected: u32) {
        let actual = calculate_sum_from_rows_1(&data);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_1, 0)]
    fn calculate_sum_from_rows_2_test<const N: usize>(#[case] data: [&[u8]; N], #[case] expected: u32) {
        let actual = calculate_sum_from_rows_2(&data);

        assert_eq!(expected, actual);
    }
}