#[cfg(test)] mod data;

pub fn find_increases(points: &[u32]) -> u32 {
    if points.len() <= 1 {
        return 0;
    }

    points
        .iter()
        .fold((0, u32::MAX), |(mut count, prev), curr| {
            if &prev < curr {
                count += 1;
            }

            (count, *curr)
        }).0
}

pub fn find_increases2(points: &[u32]) -> u32 {
    if points.len() <= 1 {
        return 0;
    }

    let mut count = 0;
    let mut prev = points[0];

    for curr in points.iter().skip(1) {
        if prev.lt(curr) {
            count += 1;
        }

        prev = *curr;
    }

    count
}

pub fn find_increases_sliding(points: &[u32]) -> u32 {
    if points.len() <= 3 {
        return 0;
    }

    let sums = get_sliding_sums(points);
    find_increases2(sums.as_slice())
}

pub fn get_sliding_sums(points: &[u32]) -> Vec<u32> {
    let mut sums = Vec::new();

    for i in 2..points.len() {
        sums.push(points[i - 2] + points[i - 1] + points[i]);
    }

    sums
}

#[cfg(test)]
mod tests {
    // use crate::shared::*;
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 0)]
    #[case(TEST_DATA_2, 0)]
    #[case(TEST_DATA_3, 1)]
    #[case(TEST_DATA_4, 0)]
    #[case(TEST_DATA_5, 7)]
    #[case(TEST_DATA_6, 1288)]
    fn find_increases_test<T: AsRef<[u32]>>(#[case] depths: T, #[case] expected: u32) {
        let result = find_increases2(depths.as_ref());
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_5, 5)]
    #[case(TEST_DATA_6, 1311)]
    fn find_increases_sliding_test<T: AsRef<[u32]>>(#[case] depths: T, #[case] expected: u32) {
        let result = find_increases_sliding(depths.as_ref());
        assert_eq!(expected, result);
    }
}