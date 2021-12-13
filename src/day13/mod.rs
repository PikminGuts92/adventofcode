#[cfg(test)] mod data;

use std::collections::{HashSet, HashMap};

pub fn fold_paper(coords: &[[u32; 2]], folds: &[(char, u32)], fold_count: Option<u32>) -> i64 {
    let mut y_size = 0;
    let mut x_size = 0;

    // Create grid (sparse matrix)
    let mut matrix = HashSet::new();

    coords
        .iter()
        .for_each(|[x, y]| {
            // Find matrix sizes
            if x_size.lt(&(x + 1)) {
                x_size = *x + 1;
            }

            if y_size.lt(&(y + 1)) {
                y_size = *y + 1;
            }

            matrix.insert((*x, *y));
        });

    let fold_count = match fold_count {
        Some(count) => count as usize,
        None => folds.len(),
    };

    for i in 0..fold_count {
        let (typ, pos) = folds[i];

        if typ.eq(&'x') {
            // Update overlapped values
            for x in 0..pos {
                let size_b_x = (x_size - 1) - x;

                for y in 0..y_size {
                    let side_b = matrix.contains(&(size_b_x, y));

                    // Add new
                    if side_b {
                        matrix.insert((x, y));
                    }

                    // Remove old
                    matrix.remove(&(size_b_x, y));
                }
            }

            // Update x size
            x_size = pos - 1;
        } else { // 'y'
            // Update overlapped values
            for y in 0..pos {
                let size_b_y = (y_size - 1) - y;

                for x in 0..x_size {
                    let side_b = matrix.contains(&(x, size_b_y));

                    // Add new
                    if side_b {
                        matrix.insert((x, y));
                    }

                    // Remove old
                    matrix.remove(&(x, size_b_y));
                }
            }

            // Update y size
            y_size = pos - 1;
        }
    }

    matrix.len() as i64
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 1, 17)]
    #[case(TEST_DATA_2_0, TEST_DATA_2_1, 1, 684)]
    pub fn fold_paper_test<T: AsRef<[[u32; 2]]>, S: AsRef<[(char, u32)]>>(#[case] coords: T, #[case] folds: S, #[case] fold_count: u32, #[case] expected: i64) {
        let result = fold_paper(coords.as_ref(), folds.as_ref(), Some(fold_count));
        assert_eq!(expected, result);
    }
}