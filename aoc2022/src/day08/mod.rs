#[cfg(test)] mod data;

pub fn find_count_of_visible_trees<const N: usize>(data: &[[u32; N]; N]) -> usize {
    let mut visible_count = 0usize;
    #[allow(non_snake_case)] let MAX_INDEX = N - 1;

    let mut visible_map = [[false; N]; N];

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

            visible_map[y][x] = is_visible;
        }
    }

    visible_count
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
}