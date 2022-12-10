#[cfg(test)] mod data;

use itertools::*;
use std::collections::{HashMap, HashSet};

pub enum Direction {
    Up,
    Right,
    Left,
    Down
}

pub fn count_tail_positions(steps: &[(Direction, i32)], knot_count: usize) -> u32 {
    let mut knots = vec![(0, 0); knot_count];
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::from_iter([knots[knot_count - 1]]);

    let sqrt_two: f64 = 2f64.sqrt();

    for (direction, distance) in steps {
        for _ in 0..*distance {
            // Update head position
            knots[0] = {
                let (x, y) = knots[0];

                match direction {
                    Direction::Up => (x, y + 1),
                    Direction::Right => (x + 1, y),
                    Direction::Left => (x - 1, y),
                    Direction::Down => (x, y - 1),
                }
            };

            let mut prev_knot = knots[0];

            // Update knot positions
            for knot in knots.iter_mut().skip(1) {
                let orig_knot = *knot;

                // Calculate distance
                let ((x1, y1), (x2, y2)) = (prev_knot, *knot);
                let (dx, dy) = (x1 - x2, y1 - y2);

                if dx.abs() < 2 && dy.abs() < 2 {
                    break;
                }

                // Update position
                *knot = (x2 + dx.signum(), y2 + dy.signum());

                prev_knot = *knot;
            }

            // Track tail position
            visited_positions.insert(knots[knot_count - 1]);
        }
    }

    visited_positions.len() as u32
}

pub fn draw_board(knots: &[(i32, i32)]) {
    const MIN_X: i32 = -13;
    const MAX_X: i32 =  13;
    const MIN_Y: i32 = -10;
    const MAX_Y: i32 =  10;

    let mut knots = knots
        .iter()
        .enumerate()
        .group_by(|(i, k)| **k)
        .into_iter()
        .map(|(k, group)| {
            let (i, _) = group.min_by_key(|(gi, _)| *gi).unwrap();

            (k, if i == 0 { 'H' } else { char::from_digit(i as u32, 10).unwrap() })
        })
        .collect::<HashMap<(i32, i32), char>>();

    for y in (MIN_X..MAX_Y).rev() {
        for x in MIN_Y..MAX_X {
            let c = knots.remove(&(x, y)).unwrap_or('*');

            if x < MAX_X {
                print!("{c} ");
            } else {
                print!("{c}");
            }
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0,      2,   13)]
    #[case(TEST_DATA_1,      2, 6354)]
    #[case(TEST_DATA_0,     10,    1)]
    #[case(TEST_DATA_0_ALT, 10,   36)]
    #[case(TEST_DATA_1,     10, 2651)]
    fn count_tail_positions_test<const N: usize>(#[case] data: [(Direction, i32); N], #[case] knots: usize, #[case] expected: u32) {
        let result = count_tail_positions(&data, knots);

        assert_eq!(expected, result);
    }
}