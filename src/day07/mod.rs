#[cfg(test)] mod data;

use std::collections::HashMap;

pub fn calc_move_cost_cheap(x1: i32, x2: i32) -> i32 {
    (x1 - x2).abs()
}

pub fn calc_move_cost_expensive(x1: i32, x2: i32) -> i32 {
    (0..(x1 - x2).abs()).map(|x| x + 1).sum()
}

pub fn crab_positions<T: Fn(i32, i32) -> i32>(positions: &[i32], calc_cost: T) -> (i32, i32) {
    let mut hashed_positions = HashMap::new();
    let mut max_pos = i32::MIN;

    for p in positions {
        if let Some(count) = hashed_positions.get_mut(p) {
            *count += 1;
        } else {
            hashed_positions.insert(*p, 1);
        }

        // Update max pos
        if p.gt(&max_pos) {
            max_pos = *p;
        }
    }

    // Get unique positions
    let mut pos_keys = hashed_positions
        .keys()
        .map(|k| *k)
        .collect::<Vec<_>>();
    pos_keys.sort();

    let mut best_pos = -1;
    let mut best_fuel = i32::MAX;

    for destination_pos in 0..(max_pos + 1) {
        let cost: i32 = pos_keys
            .iter()
            .filter(|k| k.ne(&&destination_pos))
            .map(|k| calc_cost(*k, destination_pos) * hashed_positions[k])
            .sum();

        if cost < best_fuel {
            best_pos = destination_pos;
            best_fuel = cost;
        }
    }

    (best_pos, best_fuel)
}

pub fn crab_positions_cheap(positions: &[i32]) -> (i32, i32) {
    crab_positions(positions, calc_move_cost_cheap)
}

pub fn crab_positions_expensive(positions: &[i32]) -> (i32, i32) {
    crab_positions(positions, calc_move_cost_expensive)
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 2, 37)]
    #[case(TEST_DATA_2, 376, 352707)]
    pub fn crab_positions_cheap_test<T: AsRef<[i32]>>(#[case] positions: T, #[case] expected_pos: i32, #[case] expected_cost: i32) {
        let (actual_pos, actual_cost) = crab_positions_cheap(positions.as_ref());
        assert_eq!(expected_pos, actual_pos);
        assert_eq!(expected_cost, actual_cost);
    }

    #[rstest]
    #[case(TEST_DATA_1, 5, 168)]
    #[case(TEST_DATA_2, 490, 95519693)]
    pub fn crab_positions_expensive_test<T: AsRef<[i32]>>(#[case] positions: T, #[case] expected_pos: i32, #[case] expected_cost: i32) {
        let (actual_pos, actual_cost) = crab_positions_expensive(positions.as_ref());
        assert_eq!(expected_pos, actual_pos);
        assert_eq!(expected_cost, actual_cost);
    }

    #[rstest]
    #[case(16, 5, 66)]
    #[case(5, 16, 66)]
    #[case(1, 5, 10)]
    #[case(14, 5, 45)]
    pub fn calc_move_cost_expensive_test(#[case] x1: i32, #[case] x2: i32, #[case] expected_cost: i32) {
        let actual_cost = calc_move_cost_expensive(x1, x2);
        assert_eq!(expected_cost, actual_cost);
    }
}