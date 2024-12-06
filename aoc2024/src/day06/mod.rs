use std::collections::HashSet;

#[cfg(test)] mod data;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn find_start_pos(data: &[&[u8]]) -> (usize, usize) {
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == b'^' {
                return (y, x);
            }
        }
    }

    unreachable!("^ not found!")
}

fn is_pos_within_bounds(data: &[&[u8]], (pos_y, pos_x): (usize, usize)) -> bool {
    pos_y < data.len() && pos_x < data[pos_y].len()
}

fn calculate_distinct_positions_count(data: &[&[u8]]) -> i32 {
    let mut current_pos = find_start_pos(data);
    let mut current_dir = Direction::Up;
    let mut visited = HashSet::new();

    while is_pos_within_bounds(data, current_pos) {
        visited.insert(current_pos);

        let (pos_y, pos_x) = current_pos;

        let (next_pos_y, next_pos_x) = match current_dir {
            Direction::Up => (pos_y.wrapping_sub(1), pos_x),
            Direction::Right => (pos_y, pos_x.wrapping_add(1)),
            Direction::Down => (pos_y.wrapping_add(1), pos_x),
            Direction::Left => (pos_y, pos_x.wrapping_sub(1)),
        };

        let has_obstruction = data
            .get(next_pos_y)
            .and_then(|d| d.get(next_pos_x))
            .map(|d| *d == b'#')
            .unwrap_or_default();

        if has_obstruction {
            // Update direction by rotating 90 degrees clockwise
            current_dir = match current_dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            current_pos = (next_pos_y, next_pos_x);
        }
    }

    visited.len() as i32
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 41)]
    #[case(TEST_DATA_1, 4982)]
    fn calculate_distinct_positions_count_test<const N: usize>(#[case] data: [&[u8]; N], #[case] expected: i32) {
        let actual = calculate_distinct_positions_count(&data);

        assert_eq!(expected, actual);
    }
}