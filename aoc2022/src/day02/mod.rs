#[cfg(test)] mod data;

#[derive(PartialEq)]
pub enum IndividualPlay { // Shape
    Rock,
    Paper,
    Scissors
}

impl IndividualPlay {
    pub fn from_char(c: char) -> IndividualPlay {
        match c {
            'A' | 'X' => IndividualPlay::Rock,
            'B' | 'Y' => IndividualPlay::Paper,
            'C' | 'Z' => IndividualPlay::Scissors,
            _ => unimplemented!("Character value of \'{c}\' for play not supported")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PlayResult {
    Player1,
    Player2,
    Draw
}

pub fn parse_plays(plays: &[(char, char)]) -> Vec<(IndividualPlay, IndividualPlay)> {
    plays
        .iter()
        .map(|(a, b)| (
            IndividualPlay::from_char(*a),
            IndividualPlay::from_char(*b)
        ))
        .collect()
}

pub fn calc_who_won(p1: &IndividualPlay, p2: &IndividualPlay) -> PlayResult {
    if p1.eq(p2) {
        return PlayResult::Draw;
    }

    match (p1, p2) {
        // P1 wins
        (IndividualPlay::Rock, IndividualPlay::Scissors) => PlayResult::Player1,
        (IndividualPlay::Paper, IndividualPlay::Rock) => PlayResult::Player1,
        (IndividualPlay::Scissors, IndividualPlay::Paper) => PlayResult::Player1,
        // P2 wins
        _ => PlayResult::Player2
    }
}

pub fn calc_scores(plays: &[(IndividualPlay, IndividualPlay)]) -> (u32, u32) {
    let (mut p1_score, mut p2_score) = (0u32, 0u32);

    for (p1, p2) in plays.iter() {
        let play_result = calc_who_won(p1, p2);

        // Add shape choice scores
        for (p, score) in [(p1, &mut p1_score), (p2, &mut p2_score)].iter_mut() {
            **score += match p {
                IndividualPlay::Rock => 1,
                IndividualPlay::Paper => 2,
                IndividualPlay::Scissors => 3
            }
        }

        // Add winning scores
        match play_result {
            PlayResult::Player1 => {
                p1_score += 6;
            },
            PlayResult::Player2 => {
                p2_score += 6;
            },
            PlayResult::Draw => {
                p1_score += 3;
                p2_score += 3;
            },
        }
    }

    (p1_score, p2_score)
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(IndividualPlay::Rock, IndividualPlay::Scissors, PlayResult::Player1)]
    #[case(IndividualPlay::Paper, IndividualPlay::Rock, PlayResult::Player1)]
    #[case(IndividualPlay::Scissors, IndividualPlay::Paper, PlayResult::Player1)]
    #[case(IndividualPlay::Scissors, IndividualPlay::Rock, PlayResult::Player2)]
    #[case(IndividualPlay::Rock, IndividualPlay::Paper, PlayResult::Player2)]
    #[case(IndividualPlay::Paper, IndividualPlay::Scissors, PlayResult::Player2)]
    #[case(IndividualPlay::Rock, IndividualPlay::Rock, PlayResult::Draw)]
    #[case(IndividualPlay::Paper, IndividualPlay::Paper, PlayResult::Draw)]
    #[case(IndividualPlay::Scissors, IndividualPlay::Scissors, PlayResult::Draw)]
    fn calc_who_won_test(#[case] p1: IndividualPlay, #[case] p2: IndividualPlay, #[case] expected: PlayResult) {
        let result = calc_who_won(&p1, &p2);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_0, 15)]
    #[case(TEST_DATA_1, 10718)]
    fn calc_scores_test<const N: usize>(#[case] raw_data: [(char, char); N], #[case] expected: u32) {
        let plays = parse_plays(&raw_data);
        let (_, p2_score) = calc_scores(plays.as_slice());

        assert_eq!(expected, p2_score);
    }
}