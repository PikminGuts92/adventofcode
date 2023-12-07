#[cfg(test)] mod data;

fn calculate_hand_winnings(hands: &[(&[u8], u32)]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 6440)]
    fn calculate_hand_winnings_test<const N: usize>(#[case] data: [(&[u8], u32); N], #[case] expected: u32) {
        let actual = calculate_hand_winnings(&data);

        assert_eq!(expected, actual);
    }
}