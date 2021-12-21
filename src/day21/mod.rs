#[cfg(test)] mod data;

pub fn place_dirac_game([player_1_start, player_2_start]: &[i64; 2]) -> i64 {
    // 1 die, two pawns, 10 spaces on circular board
    // Play until one pawn reaches 1000 points
    // Add value of last space landed to pawn at end of turn

    // Loser points + roll count
    0
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case([4, 8], 739785)]
    pub fn place_dirac_game_test(#[case] start_positions: [i64; 2], #[case] expected: i64) {
        let result = place_dirac_game(&start_positions);
        assert_eq!(expected, result);
    }
}