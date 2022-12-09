#[cfg(test)] mod data;

use std::collections::HashSet;

pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

pub fn find_tail_positions(steps: &[(Direction, i32)]) -> u32 {
    let (mut head_pos, mut tail_pos) = ((0, 0), (0, 0));
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::from_iter([tail_pos]);

    for (direction, distance) in steps {
        for _ in 0..*distance {
            // Update head position
            head_pos = {
                let (x, y) = head_pos;

                match direction {
                    Direction::Up => (x, y + 1),
                    Direction::Right => (x + 1, y),
                    Direction::Left => (x - 1, y),
                    Direction::Down => (x, y - 1),
                }
            };

            // Update tail position

            // Track tail position
            visited_positions.insert(tail_pos);
        }
    }

    visited_positions.len() as u32
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 13)]
    #[case(TEST_DATA_1, 0)]
    fn find_tail_positionss_test<const N: usize>(#[case] data: [(Direction, i32); N], #[case] expected: u32) {
        let result = find_tail_positions(&data);

        assert_eq!(expected, result);
    }
}