#[cfg(test)] mod data;

pub struct DieBlock {
    value: Option<i64>,
    max_value: i64,
}

impl DieBlock {
    pub fn new(size: i64) -> DieBlock {
        DieBlock {
            value: None,
            max_value: size
        }
    }

    pub fn roll(&mut self) -> i64 {
        if let Some(value) = &mut self.value {
            if value == &self.max_value {
                *value = 1;
            } else {
                *value += 1;
            }

            return *value
        } else {
            self.value = Some(1);
            return 1
        }
    }
}

pub fn place_dirac_game([p1_start, p2_start]: &[i64; 2]) -> i64 {
    // 1 die, two pawns, 10 spaces on circular board
    // Play until one pawn reaches 1000 points
    // Add value of last space landed to pawn at end of turn
    // Roll die 3 times each turn

    const BOARD_SIZE: i64 = 10;
    const DIE_SIZE: i64 = 100;
    const MAX_POINTS: i64 = 1000;

    let mut move_count = 0i64;
    let mut roll_count = 0i64;
    let mut points = [0i64; 2];
    let mut positions = [*p1_start - 1, *p2_start - 1];

    let mut die = DieBlock::new(DIE_SIZE);

    loop {
        let player = (move_count & 0b1) as usize;
        let roll_move = die.roll() + die.roll() + die.roll();

        // Update roll count
        roll_count += 3;

        // Update pos
        let new_pos = (positions[player] + roll_move) % BOARD_SIZE;
        positions[player] = new_pos;

        // Update points + move count
        points[player] += new_pos + 1;
        move_count += 1;

        if points[player].ge(&MAX_POINTS) {
            break;
        }
    }

    // Loser points + roll count
    points[(move_count & 0b1) as usize] * roll_count
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case([4, 8], 739785)]
    #[case([4, 6], 888735)]
    pub fn place_dirac_game_test(#[case] start_positions: [i64; 2], #[case] expected: i64) {
        let result = place_dirac_game(&start_positions);
        assert_eq!(expected, result);
    }
}