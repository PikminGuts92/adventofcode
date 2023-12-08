use std::cmp::Ordering;

#[cfg(test)] mod data;

fn get_card_index(card: u8) -> usize {
    match card {
        n @ b'2'..=b'9'=> ((n - b'0') - 2) as usize,
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unreachable!()
    }
}

fn get_card_index_joker(card: u8) -> usize {
    match card {
        b'J' => 0,
        n @ b'2'..=b'9'=> ((n - b'0') - 1) as usize,
        b'T' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unreachable!()
    }
}

fn compute_hand_type(hand: &[u8; 5], joker: bool) -> u32 {
    let mut cards = [0u8; 13]; // Maybe allocate beforehand?
    let mut joker_count = 0;

    for card in hand {
        if joker && *card == b'J' {
            joker_count += 1;
            continue;
        }

        let idx = get_card_index(*card);
        cards[idx] = (cards[idx] << 1) | 0b1;
    }

    //println!("{cards:?}\n");

    let mut hand_score = 0;

    #[allow(unused)]
    let (mut one_pair, mut two_pair, mut triple) = (false, false, false);

    for card_score in cards {
        /*let modified_score = if joker_count > 0 {
            ((card_score << joker_count) & 0b11111) | (1 << (joker_count - 1))
        } else {
            card_score
        };*/

        let score = match card_score {
            0b00000 => 0,
            0b00001 => 1,
            0b00011 => {
                two_pair = one_pair;
                one_pair = true;

                if triple {
                    // Full-house
                    5
                } else if two_pair {
                    3
                } else {
                    2
                }
            },
            0b00111 => {
                triple = true;

                if one_pair {
                    // Full-house
                    5
                } else {
                    4
                }
            },
            0b01111 => 6,
            0b11111 => 7,
            //_ => unreachable!()
            n @ _ => {
                println!("Unexpected: {n}");
                unreachable!()
            }
        };

        hand_score = hand_score.max(score);
    }

    if joker && joker_count > 0 {
        /*const SINGLE: u32     = 1;
        const PAIR: u32       = 2;
        const TWO_PAIR: u32   = 3;
        const THREE: u32      = 4;
        const FULL_HOUSE: u32 = 5;
        const FOUR: u32       = 6;
        const FIVE: u32       = 7;*/

        //println!("Jokers: {joker_count}");

        hand_score = match (hand_score, joker_count) {
            (1, 1) => 2, // Single -> Pair
            (2, 1) => 4, // Pair -> Three
            (3, 1) => 5, // Two pair -> Full-house
            (4, 1) => 6, // Three -> Four
            //(5, 1) => 6, // Full-house -> Four
            (6, 1) => 7, // Four -> Five

            (1, 2) => 4, // Single -> Three
            (2, 2) => 6, // Pair -> Four
            //(3, 2) => 5, // Two pair
            (4, 2) => 7, // Three -> Five

            (1, 3) => 6, // Single -> Four
            (2, 3) => 7, // Pair -> Five

            (1, 4) => 7, // Single -> Five

            (0, 5) => 7, // All jokers -> Five
            _ => unreachable!()
        };
    }

    hand_score
}

fn calculate_hand_winnings(hands: &[(&[u8; 5], u32)], joker: bool) -> u32 {
    let mut computed_scores = hands
        .iter()
        .map(|(hand, wager)| {
            let hand_type = compute_hand_type(hand, joker);
            (*hand, *wager, hand_type)
        })
        .collect::<Vec<_>>();

    let get_card_index_local = if joker {
        get_card_index_joker
    } else {
        get_card_index
    };

    // Sort scores
    computed_scores
        .sort_by(|(a_hand, _, a_score), (b_hand, _, b_score)| {
            if a_score == b_score {
                // Compare hands
                for i in 0..a_hand.len() {
                    let ah = a_hand[i];
                    let bh = b_hand[i];

                    // Don't bother indexing if equal
                    if ah == bh {
                        continue;
                    }

                    // Compare index (score)
                    let ai = get_card_index_local(ah);
                    let bi = get_card_index_local(bh);

                    return ai.cmp(&bi);
                }

                return Ordering::Equal;
            }

            a_score.cmp(b_score)
        });

    /*for (hand, wager) in hands {
        let hand_type = compute_hand_type(hand);
        println!("Hand: {:?}", String::from_utf8(hand.to_vec()).unwrap());
        println!("Score: {hand_type}\n\n");
        // current_score += hand_type * wager;
    }*/

    //println!("{computed_scores:#?}");

    computed_scores
        .into_iter()
        .enumerate()
        .map(|(i, (_, wager, _))| (i as u32 + 1) * wager)
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, false, 6440)]
    #[case(TEST_DATA_1, false, 251216224)]
    #[case(TEST_DATA_0, true, 5905)]
    #[case(TEST_DATA_1, true, 250825971)]
    fn calculate_hand_winnings_test<const N: usize>(#[case] data: [(&[u8; 5], u32); N], #[case] joker: bool,  #[case] expected: u32) {
        let actual = calculate_hand_winnings(&data, joker);

        assert_eq!(expected, actual);
    }
}