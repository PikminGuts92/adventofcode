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
    let mut prev_head_pos;

    let sqrt_two: f64 = 2f64.sqrt();

    for (direction, distance) in steps {
        for _ in 0..*distance {
            prev_head_pos = head_pos;

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
            let ((x1, y1), (x2, y2)) = (head_pos, tail_pos);
            let d = (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt();
            //println!("Distance is {d}");

            if d > sqrt_two {
                tail_pos = prev_head_pos;
            }

            //let ((x1, y1), (x2, y2)) = (head_pos, tail_pos);
            //let d = (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt();
            //println!("NEW Distance is {d}");

            // Track tail position
            visited_positions.insert(tail_pos);

            // DEBUG
            //draw_board(head_pos, tail_pos);
            //println!();
        }
    }

    visited_positions.len() as u32
}

pub fn draw_board(head_pos: (i32, i32), tail_pos: (i32, i32)) {
    let ((x1, y1), (x2, y2)) = (head_pos, tail_pos);

    for y in (0..5).rev() {
        for x in 0..6 {
            let c = match (head_pos, tail_pos) {
                ((x1, y1), _) if x == x1 && y == y1  => 'H',
                (_, (x2, y2)) if x == x2 && y == y2  => 'T',
                _ => '*'
            };

            if x < 6 {
                print!("{c} ");
            } else {
                print!("{c} ");
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
    #[case(TEST_DATA_0, 13)]
    #[case(TEST_DATA_1, 6354)]
    fn find_tail_positionss_test<const N: usize>(#[case] data: [(Direction, i32); N], #[case] expected: u32) {
        let result = find_tail_positions(&data);

        assert_eq!(expected, result);
    }
}