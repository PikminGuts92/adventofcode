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

pub fn place_dirac_game_part2([p1_start, p2_start]: &[i64; 2]) -> i64 {
    const BOARD_SIZE: usize = 10;
    const MAX_POINTS: usize = 21;

    // Frequency of results from 3-sided 3 dice roll
    const MOVES: [(i64, i64); 7] = [
        (3, 1),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1)
    ];

    let mut points = [0i64; 2];
    let start_pos = [*p1_start, *p2_start];

    let mut space_points = [0i64; BOARD_SIZE]; // Points each space are worth
    let mut tracked_points = [0i64; MAX_POINTS + 1];

    // Iterate over players
    for p in 0..=1 {
        // Populate space points
        space_points
            .iter_mut()
            .enumerate()
            .for_each(|(i, s)| {
                let n = (i as i64) + start_pos[p];

                *s = if n > (BOARD_SIZE as i64) {
                    n - (BOARD_SIZE as i64)
                } else {
                    n
                };
            });

        // Populate initial state
        tracked_points[0] = 1;

        // Iterate over score states
        for i in 0..21 {
            play_move(i, &MOVES, &space_points, &mut tracked_points);
        }

        points[p] = tracked_points
            .last()
            .map(|tp| *tp)
            .unwrap_or_default();

        // Reset tracked points
        tracked_points
            .iter_mut()
            .for_each(|tp| *tp = 0);
    }

    i64::max(points[0], points[1])
}

pub fn play_move(curr_points: i64, moves: &[(i64, i64); 7], space_points: &[i64], tracked_points: &mut [i64]) {
    let curr_pos = curr_points % (space_points.len() as i64);
    let curr_freq = tracked_points[curr_points as usize];

    for (m_stride, m_freq) in moves.iter() {
        let next_pos = (curr_pos + m_stride) % (space_points.len() as i64);
        let mut next_points = (curr_points + space_points[next_pos as usize]) as usize;

        if next_points > 21 {
            next_points = 21;
        }

        tracked_points[next_points] += curr_freq * *m_freq;
    }

    // Clear current
    tracked_points[curr_points as usize] = 0;
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

    #[rstest]
    #[case([4, 8], 444356092776315)]
    #[case([4, 6], 0)]
    pub fn place_dirac_game_part2_test(#[case] start_positions: [i64; 2], #[case] expected: i64) {
        let result = place_dirac_game_part2(&start_positions);
        assert_eq!(expected, result);
    }
}