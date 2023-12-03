#[cfg(test)] mod data;

fn find_numbers_in_row(row: &[u8]) -> Vec<(u32, usize, usize)> {
    let mut numbers = Vec::new();
    let mut start = None;
    let mut x = 0;

    while let Some(c) = row.get(x).map(|c| *c) {
        let is_digit = c.is_ascii_digit();

        if is_digit && start.is_none() {
            start = Some(x);
        }

        // Peek at next character
        let is_next_digit = row.get(x + 1).map(|c| c.is_ascii_digit()).unwrap_or_default();

        if !is_next_digit && start.is_some() {
            let start = start.take().unwrap();
            let end = x;

            let number = std::str::from_utf8(&row[start..(end + 1)]).unwrap().parse::<u32>().unwrap();
            numbers.push((number, start, end));
        }

        x += 1;
    }

    numbers
}

fn check_adjacent_for_symbol(y: usize, x: usize, data: &[&[u8]]) -> bool {
    let surroundings = [
        (Some(y), x.checked_sub(1)),
        (Some(y), x.checked_add(1)),
        (y.checked_sub(1), Some(x)),
        (y.checked_add(1), Some(x)),
        (y.checked_sub(1), x.checked_sub(1)),
        (y.checked_add(1), x.checked_add(1)),
        (y.checked_sub(1), x.checked_add(1)),
        (y.checked_add(1), x.checked_sub(1)),
    ];

    for (y_pos, x_pos) in surroundings {
        let Some(dy) = y_pos.and_then(|y| data.get(y)) else {
            continue;
        };

        let Some(dx) = x_pos.and_then(|x| dy.get(x)) else {
            continue;
        };

        if *dx != b'.' && !dx.is_ascii_digit() {
            return true;
        }
    }

    false
}

fn find_sum_of_adjacent_numbers(data: &[&[u8]]) -> u32 {
    let mut current_sum = 0;

    for y in 0..data.len() {
        let numbers = find_numbers_in_row(data[y]);

        println!("{numbers:?}"); // TODO: Remove

        for (number, start_pos, end_pos) in numbers {
            // Check start pos
            if check_adjacent_for_symbol(y, start_pos, &data) {
                current_sum += number;
                continue;
            }

            // Check end pos
            if check_adjacent_for_symbol(y, end_pos, &data) {
                current_sum += number;
            }
        }
    }

    current_sum
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 4361)]
    #[case(TEST_DATA_1, 521515)]
    fn dummy_test<const N: usize>(#[case] data: [&[u8]; N], #[case] expected: u32) {
        let actual = find_sum_of_adjacent_numbers(&data);

        assert_eq!(expected, actual);
    }
}