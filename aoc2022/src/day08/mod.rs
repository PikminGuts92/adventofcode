#[cfg(test)] mod data;

use take_until::*;

pub fn find_count_of_visible_trees<const N: usize>(data: &[[u32; N]; N]) -> usize {
    let mut visible_count = 0usize;
    #[allow(non_snake_case)] let MAX_INDEX = N - 1;

    for y in 0..N {
        for x in 0..N {
            let tree_height = data[y][x];

            #[allow(unused_parens)]
            let is_visible = (
                ((y == 0)         || (0..y).rev().all(|yy| data[yy][x].lt(&tree_height))) || // Top
                ((x == 0)         || (0..x).rev().all(|xx| data[y][xx].lt(&tree_height))) || // Left
                ((x == MAX_INDEX) || (x..N).skip(1).all(|xx| data[y][xx].lt(&tree_height))) || // Right
                ((y == MAX_INDEX) || (y..N).skip(1).all(|yy| data[yy][x].lt(&tree_height))) // Down
            );

            if is_visible {
                visible_count += 1;
            }
        }
    }

    visible_count
}

pub fn find_highest_scenic_score<const N: usize>(data: &[[u32; N]; N]) -> usize {
    let mut best_score = usize::MIN;

    // Skip outer trees
    for y in 1..(N - 1) {
        for x in 1..(N - 1) {
            let tree_height = data[y][x];

            #[allow(unused_parens)]
            let scenic_score = (
                (0..y).rev().take_until(|yy| data[*yy][x].ge(&tree_height)).count() * // Top
                (0..x).rev().take_until(|xx| data[y][*xx].ge(&tree_height)).count() * // Left
                (x..N).skip(1).take_until(|xx| data[y][*xx].ge(&tree_height)).count() * // Right
                (y..N).skip(1).take_until(|yy| data[*yy][x].ge(&tree_height)).count() // Down
            );

            best_score = best_score.max(scenic_score);
        }
    }

    best_score
}


#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 21)]
    #[case(TEST_DATA_1, 1859)]
    fn find_count_of_visible_trees_test<const N: usize>(#[case] data: [[u32; N]; N], #[case] expected: usize) {
        let result = find_count_of_visible_trees(&data);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_0, 8)]
    #[case(TEST_DATA_1, 332640)]
    fn find_highest_scenic_score_test<const N: usize>(#[case] data: [[u32; N]; N], #[case] expected: usize) {
        let result = find_highest_scenic_score(&data);

        assert_eq!(expected, result);
    }
}