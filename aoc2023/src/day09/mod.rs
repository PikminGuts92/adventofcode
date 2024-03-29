#[cfg(test)] mod data;

fn find_num_diffs(data: &[i64]) -> Vec<i64> {
    data
        .iter()
        .take(data.len() - 1)
        .zip(data.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>()
}

fn extrapolate_num(data: &[i64]) -> i64 {
    let mut sequences = vec![
        data.iter().map(|d| *d).collect::<Vec<_>>()
    ];

    // Find all sequences until 0s
    loop {
        let last_seq = sequences.last().unwrap();
        if last_seq.iter().all(|s| s.eq(&0)) {
            break;
        }

        let diffs = find_num_diffs(last_seq);
        sequences.push(diffs);
    }

    for i in (1..(sequences.len())).into_iter().rev() {
        let current_row_num = sequences[i].last().map(|n| *n).unwrap();
        let next_row_num = sequences[i - 1].last().map(|n| *n).unwrap();

        sequences[i - 1].push(current_row_num + next_row_num);
    }

    sequences
        .first()
        .and_then(|s| s.last())
        .map(|n| *n)
        .unwrap()
}

fn extrapolate_sum<const M: usize>(data: &[[i64; M]]) -> i64 {
    data
        .iter()
        .map(|row| extrapolate_num(row))
        .sum()
}

fn find_num_diffs_left(data: &[i64]) -> Vec<i64> {
    data
        .iter()
        .take(data.len() - 1)
        .zip(data.iter().skip(1))
        .map(|(a, b)| a - b)
        .collect::<Vec<_>>()
}

fn extrapolate_num_left(data: &[i64]) -> i64 {
    let mut sequences = vec![
        data.iter().map(|d| *d).collect::<Vec<_>>()
    ];

    // Find all sequences until 0s
    loop {
        let last_seq = sequences.last().unwrap();
        if last_seq.iter().all(|s| s.eq(&0)) {
            break;
        }

        let diffs = find_num_diffs_left(last_seq);
        sequences.push(diffs);
    }

    for i in (1..(sequences.len())).into_iter().rev() {
        let current_row_num = sequences[i].first().map(|n| *n).unwrap();
        let next_row_num = sequences[i - 1].first().map(|n| *n).unwrap();

        sequences[i - 1].insert(0, current_row_num + next_row_num);
    }

    sequences
        .first()
        .and_then(|s| s.first())
        .map(|n| *n)
        .unwrap()
}

fn extrapolate_sum_left<const M: usize>(data: &[[i64; M]]) -> i64 {
    data
        .iter()
        .map(|row| extrapolate_num_left(row))
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 114)]
    #[case(TEST_DATA_1, 1681758908)]
    fn extrapolate_sum_test<const N: usize, const M: usize>(#[case] data: [[i64; M]; N], #[case] expected: i64) {
        let actual = extrapolate_sum(&data);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_0, 2)]
    #[case(TEST_DATA_1, 803)]
    fn extrapolate_sum_left_test<const N: usize, const M: usize>(#[case] data: [[i64; M]; N], #[case] expected: i64) {
        let actual = extrapolate_sum_left(&data);

        assert_eq!(expected, actual);
    }
}