#[cfg(test)] mod data;

use std::collections::HashSet;

pub fn find_index_of_char<const M: usize>(data: &[[char; M]], c: char, ) -> (usize, usize) {
    let (x, y, _) = data
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row
            .iter()
            .enumerate()
            .map(move |(x, rc)| (x, y, rc))
        )
        .filter(|(x, y, rc)| c.eq(rc))
        .next()
        .unwrap();

    (x, y)
}

pub fn find_indicies_of_char<const M: usize>(data: &[[char; M]], c: char, ) -> Vec<(usize, usize)> {
    data
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row
            .iter()
            .enumerate()
            .map(move |(x, rc)| (x, y, rc))
        )
        .filter(|(x, y, rc)| c.eq(rc))
        .map(|(x, y, _)| (x, y))
        .collect()
}

pub fn find_shortest_path<const M: usize, const N: usize>(data: &[[char; M]; N]) -> i32 {
    // Get start/end positions
    let (start_x, start_y) = find_index_of_char(data, 'S');
    let (end_x, end_y) = find_index_of_char(data, 'E');

    let mut best_scores = [[i32::MAX; M]; N];

    progress_terrain(data, &mut best_scores, (start_x, start_y), (end_x, end_y), &mut HashSet::new());

    best_scores[end_y][end_x]
}

pub fn find_shortest_path_for_a<const M: usize, const N: usize>(data: &[[char; M]; N]) -> i32 {
    // Get start/end positions
    let start_positions = find_indicies_of_char(data, 'a');
    let (end_x, end_y) = find_index_of_char(data, 'E');

    let mut best_scores = [[i32::MAX; M]; N];

    for (start_x, start_y) in start_positions {
        progress_terrain(data, &mut best_scores, (start_x, start_y), (end_x, end_y), &mut HashSet::new());

    }

    best_scores[end_y][end_x]
}

pub fn progress_terrain<const M: usize, const N: usize>(data: &[[char; M]; N], scores: &mut [[i32; M]; N], (pos_x, pos_y): (usize, usize), (end_x, end_y): (usize, usize), path: &mut HashSet<(usize, usize)>) {
    let best_score = scores[end_y][end_x];
    let current_score = path.len() as i32;

    // Update score
    scores[pos_y][pos_x] = if scores[pos_y][pos_x].gt(&current_score)
        && current_score < best_score {
        current_score
    } else {
        return;
    };

    // Calculate current height
    let height = match data[pos_y][pos_x] {
        'E' => {
            // End is found
            return;
        },
        'S' => ('a' as i8) - ('a' as i8),
        c @ _ => (c as i8) - ('a' as i8)
    };

    path.insert((pos_x, pos_y));

    // Get next positions
    let next_positions = [(0, -1), (0, 1), (1, 0), (-1, 0)]
        .into_iter()
        .map(|(x, y)| match (pos_x.checked_add_signed(x), pos_y.checked_add_signed(y)) {
            (Some(x), Some(y)) => data
                .get(y)
                .and_then(|r| r.get(x))
                .and_then(|c| match c {
                    'E' => Some(('z' as i8) - ('a' as i8)),
                    'S' => None, //Some(i8::MAX),
                    c @ _ => Some((*c as i8) - ('a' as i8))
                })
                .map(|h| (h, (x, y))),
            _ => None
        })
        .flatten()
        .filter(|(h, (x, y))| !path.contains(&(*x, *y)) && (h.le(&height) || (height + 1).eq(h)))
        .map(|(_, pos)| pos)
        .collect::<Vec<_>>();

    // Recursively search
    for next_pos in next_positions {
        progress_terrain(data, scores, next_pos, (end_x, end_y), path);
    }

    path.remove(&(pos_x, pos_y));
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 31)]
    #[case(TEST_DATA_1, 352)]
    fn find_shortest_path_test<const M: usize, const N: usize>(#[case] data: [[char; M]; N], #[case] expected: i32) {
        let result = find_shortest_path(&data);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_0, 29)]
    #[case(TEST_DATA_1, 345)]
    fn find_shortest_path_for_a_test<const M: usize, const N: usize>(#[case] data: [[char; M]; N], #[case] expected: i32) {
        let result = find_shortest_path_for_a(&data);

        assert_eq!(expected, result);
    }
}