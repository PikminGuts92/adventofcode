#[cfg(test)] mod data;

pub fn simulate_population_growth(state: &[u32], mut days: u32) -> u64 {
    let mut fish_counts = [0u64; 9];

    for s in state {
        fish_counts[*s as usize] += 1;
    }

    while days > 0 {
        let start_count = fish_counts[0];

        // Update days
        for i in 0..(fish_counts.len() - 1) {
            fish_counts[i] = fish_counts[i + 1];
        }
        fish_counts[6] += start_count;

        // Spawn new fish
        fish_counts[8] = start_count;

        days -= 1;
    }

    fish_counts
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 18, 26)]
    #[case(TEST_DATA_1, 80, 5934)]
    #[case(TEST_DATA_1, 256, 26984457539)]
    #[case(TEST_DATA_2, 80, 391888)]
    #[case(TEST_DATA_2, 256, 1754597645339)]
    pub fn simulate_population_growth_test<T: AsRef<[u32]>>(#[case] state: T, #[case] days: u32, #[case] expected: u64) {
        let result = simulate_population_growth(state.as_ref(), days);
        assert_eq!(expected, result);
    }
}