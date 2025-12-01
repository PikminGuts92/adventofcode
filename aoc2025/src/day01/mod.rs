use std::collections::HashMap;

#[cfg(test)] mod data;

fn count_zero_dial_positions_every_rotation(data: &[&str]) -> i32 {
    let mut pos = 50;
    let mut zero_count = 0;

    for dial_move in data {
        let (dir_raw, count_raw) = dial_move.split_at(1);

        let dir = if dir_raw.eq("L") { -1 } else { 1 };
        let count = count_raw.parse::<i32>().unwrap();

        //let mut local_zero_count = 0;

        /*while count > 0 {
            let min_mov = i32::min(100, count);
            pos += dir * min_mov;
            count -= min_mov;

            if pos >= 100 {
                zero_count += 1;
                local_zero_count += 1;

                pos -= 100;
            } else if pos < 0 {
                if pos != -min_mov {
                    zero_count += 1;
                    local_zero_count += 1;
                }

                pos += 100;
            } else if pos == 0 {
                zero_count += 1;
            }
        }*/

        //let start_pos = pos;
        pos += dir * count;

        while pos < 0 || pos >= 100 {
            if pos != -count && pos != 100 {
                //local_zero_count += 1;
                zero_count += 1;
            }

            pos += 100 * -dir;
        }

        if pos == 0 {
            zero_count += 1;
        }

        /*if local_zero_count > 0 {
            let zero_count_name = match local_zero_count {
                1 => String::from("once"),
                2 => String::from("twice"),
                n => format!("{n} times")
            };

            println!("The dial is rotated {dial_move} to point at {pos}; during this rotation, it points at 0 {zero_count_name}");
        } else {
            println!("The dial is rotated {dial_move} to point at {pos}");
        }*/
    }

    zero_count
}

fn count_zero_dial_positions(data: &[&str]) -> i32 {
    let mut pos = 50;
    let mut zero_count = 0;

    for dial_move in data {
        let (dir_raw, count_raw) = dial_move.split_at(1);

        let dir = if dir_raw.eq("L") { -1 } else { 1 };
        let count = count_raw.parse::<i32>().unwrap();

        pos += dir * count;

        while pos < 0 || pos >= 100 {
            pos += 100 * -dir;
        }

        //println!("The dial is rotated {dial_move} to point at {pos}");
        if pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 3)]
    #[case(TEST_DATA_1, 964)]
    fn count_zero_dial_positions_test<const N: usize>(#[case] data: [&str; N], #[case] expected: i32) {
        let dial_pos = count_zero_dial_positions(&data);
        assert_eq!(expected, dial_pos);
    }

    #[rstest]
    #[case(TEST_DATA_0, 6)]
    #[case(TEST_DATA_1, 5872)]
    fn count_zero_dial_positions_every_rotation_test<const N: usize>(#[case] data: [&str; N], #[case] expected: i32) {
        let dial_pos = count_zero_dial_positions_every_rotation(&data);
        assert_eq!(expected, dial_pos);
    }
}