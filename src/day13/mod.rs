#[cfg(test)] mod data;

use std::collections::HashSet;

pub fn fold_paper(coords: &[[u32; 2]], folds: &[(char, u32)], fold_count: Option<u32>, print: bool) -> i64 {
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
                let side_b_x = (pos * 2) - x;

                for y in 0..y_size {
                    let side_b = matrix.contains(&(side_b_x, y));

                    // Add new
                    if side_b {
                        matrix.insert((x, y));
                    }

                    // Remove old
                    matrix.remove(&(side_b_x, y));
                }
            }

            // Remove crease points
            for y in 0..y_size {
                matrix.remove(&(pos, y));
            }

            // Update x size
            x_size = pos;
        } else { // 'y'
            // Update overlapped values
            for y in 0..pos {
                let side_b_y = (pos * 2) - y;

                for x in 0..x_size {
                    let side_b = matrix.contains(&(x, side_b_y));

                    // Add new
                    if side_b {
                        matrix.insert((x, y));
                    }

                    // Remove old
                    matrix.remove(&(x, side_b_y));
                }
            }

            // Remove crease points
            for x in 0..x_size {
                matrix.remove(&(x, pos));
            }

            // Update y size
            y_size = pos;
        }
    }

    if print {
        print_matrix(&matrix, x_size, y_size);
    }

    matrix.len() as i64
}

pub fn print_matrix(matrix: &HashSet<(u32, u32)>, x_size: u32, y_size: u32) {
    for y in 0..y_size {
        for x in 0..x_size {
            let c = match matrix.contains(&(x, y)) {
                true => '#',
                _ => ' '
            };

            print!("{}", c);
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, Some(1), false, 17)]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, None, false, 16)]
    #[case(TEST_DATA_2_0, TEST_DATA_2_1, Some(1), false, 684)]
    #[case(TEST_DATA_2_0, TEST_DATA_2_1, None, true, 98)] // JRZBLGKH (see console output)
    pub fn fold_paper_test<T: AsRef<[[u32; 2]]>, S: AsRef<[(char, u32)]>>(#[case] coords: T, #[case] folds: S, #[case] fold_count: Option<u32>, #[case] print_output: bool, #[case] expected: i64) {
        let result = fold_paper(coords.as_ref(), folds.as_ref(), fold_count, print_output);
        assert_eq!(expected, result);
    }
}