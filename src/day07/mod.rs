#[cfg(test)] mod data;

use std::collections::HashMap;

pub fn crab_positions(positions: &[i32]) -> (i32, i32) {
    let mut hashed_positions = HashMap::new();

    for p in positions {
        if let Some(count) = hashed_positions.get_mut(p) {
            *count += 1;
        } else {
            hashed_positions.insert(*p, 1);
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

    for key_idx in 0..pos_keys.len() {
        let destination_pos = pos_keys[key_idx];

        let cost: i32 = pos_keys
            .iter()
            .filter(|k| k.ne(&&destination_pos))
            .map(|k| (k - destination_pos).abs() * hashed_positions[k])
            .sum();

        if cost < best_fuel {
            best_pos = destination_pos;
            best_fuel = cost;
        }
    }

    (best_pos, best_fuel)
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 2, 37)]
    #[case(TEST_DATA_2, 376, 352707)]
    pub fn crab_positions_test<T: AsRef<[i32]>>(#[case] positions: T, #[case] expected_pos: i32, #[case] expected_cost: i32) {
        let (actual_pos, actual_cost) = crab_positions(positions.as_ref());
        assert_eq!(expected_pos, actual_pos);
        assert_eq!(expected_cost, actual_cost);
    }
}