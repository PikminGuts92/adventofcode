#[cfg(test)] mod data;

const XMAS_TEXT: &str = "XMAS";
const XMAS_TEXT_REVERSED: &str = "SAMX";

fn get_location_count(data: &[&str], y: usize, x: usize, locations: Vec<(usize, usize)>) -> i32 {
    if locations.len() < XMAS_TEXT.len() {
        return 0;
    }

    let mut has_text = true;
    let mut has_text_reversed = true;

    for (i, (yy, xx)) in locations.into_iter().enumerate() {
        if !has_text && !has_text_reversed {
            return 0;
        }

        let current_char = &data[yy][xx..(xx + 1)];

        if has_text {
            let xmas_char = &XMAS_TEXT[i..(i + 1)];
            has_text = current_char == xmas_char;
        }

        if has_text_reversed {
            let xmas_char = &XMAS_TEXT_REVERSED[i..(i + 1)];
            has_text_reversed = current_char == xmas_char;
        }
    }

    [has_text, has_text_reversed]
        .into_iter()
        .filter(|h| *h)
        .count() as i32
}

fn check_right(data: &[&str], y: usize, x: usize) -> i32 {
    let locations = (x..(x + XMAS_TEXT.len()))
        .filter(|xx| *xx < data[y].len())
        .map(|xx| (y, xx))
        .collect::<Vec<_>>();

    get_location_count(data, y, x, locations)
}

fn check_down(data: &[&str], y: usize, x: usize) -> i32 {
    let locations = (y..(y + XMAS_TEXT.len()))
        .filter(|yy| *yy < data.len())
        .map(|yy| (yy, x))
        .collect::<Vec<_>>();

    get_location_count(data, y, x, locations)
}

fn check_diag_right_down(data: &[&str], y: usize, x: usize) -> i32 {
    let locations = (x..(x + XMAS_TEXT.len()))
        .enumerate()
        .map(|(i, xx)| (y + i, xx))
        .filter(|(yy, xx)| *yy < data.len() && *xx < data[*yy].len())
        .collect::<Vec<_>>();

    get_location_count(data, y, x, locations)
}

fn check_diag_right_up(data: &[&str], y: usize, x: usize) -> i32 {
    let locations = (x..(x + XMAS_TEXT.len()))
        .enumerate()
        .map(|(i, xx)| (y.wrapping_sub(i), xx))
        .filter(|(yy, xx)| *yy < data.len() && *xx < data[*yy].len())
        .collect::<Vec<_>>();

    get_location_count(data, y, x, locations)
}

fn find_xmas_count(data: &[&str]) -> i32 {
    let mut xmas_count = 0;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            // Check right
            let r_res = check_right(data, y, x);
            xmas_count += r_res;

            // Check down
            let d_res = check_down(data, y, x);
            xmas_count += d_res;

            // Check diag right down
            let drd_res = check_diag_right_down(data, y, x);
            xmas_count += drd_res;

            // Check diag right up
            let dru_res = check_diag_right_up(data, y, x);
            xmas_count += dru_res;
        }
    }

    xmas_count
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 18)]
    #[case(TEST_DATA_1, 2521)]
    fn find_xmas_count_test<const N: usize>(#[case] data: [&str; N], #[case] expected: i32) {
        let actual = find_xmas_count(&data);

        assert_eq!(expected, actual);
    }
}