#[cfg(test)] mod data;

use std::collections::HashSet;

pub struct BingoBoard<'a> {
    board: &'a [[u32; 5]; 5],
    matches: HashSet<(usize, usize)>,
    winning_score: Option<u32>,
}

impl<'a> BingoBoard<'a> {
    pub fn from(board: &'a [[u32; 5]; 5]) -> BingoBoard {
        BingoBoard {
            board,
            matches: HashSet::new(),
            winning_score: None,
        }
    }

    pub fn play_draw(&mut self, draw: u32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col] == draw {
                    // Update score
                    self.matches.insert((row, col));
                    self.check_win(draw);

                    // Exit
                    return;
                }
            }
        }
    }

    fn get_sum_of_unmarked_numbers(&self) -> u32 {
        let mut sum = 0;

        for row in 0..5 {
            for col in 0..5 {
                if !self.matches.contains(&(row, col)) {
                    sum += self.board[row][col];
                }
            }
        }

        sum
    }

    fn check_win(&mut self, draw: u32) {
        // Check horizontal wins
        for row in 0..5 {
            let mut count = 0;

            for col in 0..5 {
                if self.matches.contains(&(row, col)) {
                    count += 1;
                }
            }

            if count == 5 {
                //let row_sum = self.board[row].iter().sum::<u32>();
                //self.winning_score = Some(row_sum * draw);

                let sum = self.get_sum_of_unmarked_numbers();
                self.winning_score = Some(sum * draw);
                return;
            }
        }

        // Check vertical wins
        for col in 0..5 {
            let mut count = 0;

            for row in 0..5 {
                if self.matches.contains(&(row, col)) {
                    count += 1;
                }
            }

            if count == 5 {
                //let col_sum = self.board.iter().map(|r| r[col]).sum::<u32>();
                //self.winning_score = Some(col_sum * draw);

                let sum = self.get_sum_of_unmarked_numbers();
                self.winning_score = Some(sum * draw);
                return;
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        self.winning_score.is_some()
    }

    pub fn get_score(&self) -> u32 {
        self.winning_score.unwrap_or_default()
    }
}

pub fn play_bingo(draws: &[u32], boards: &[[[u32; 5]; 5]]) -> u32 {
    let mut boards = boards
        .iter()
        .map(|b| BingoBoard::from(b))
        .collect::<Vec<BingoBoard>>();

    let mut winner = -1;

    for (draw_i, draw) in draws.iter().enumerate() {
        for (board_i, board) in boards.iter_mut().enumerate() {
            board.play_draw(*draw);

            if draw_i >= 4 && board.is_winner() {
                winner = board_i as i32;
                break;
            }
        }

        if winner >= 0 {
            break;
        }
    }

    // Return score of winning board
    boards[winner as usize].get_score()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1_DRAWS, TEST_DATA_1_BOARDS, 4512)]
    #[case(TEST_DATA_2_DRAWS, TEST_DATA_2_BOARDS, 60368)]
    fn calculate_power_consumption_test<T: AsRef<[u32]>, S: AsRef<[[[u32; 5]; 5]]>>(#[case] draws: T, #[case] boards: S, #[case] expected: u32) {
        let result = play_bingo(draws.as_ref(), boards.as_ref());
        assert_eq!(expected, result);
    }
}