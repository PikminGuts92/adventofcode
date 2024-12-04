#[cfg(test)] mod data;

const XMAS_TEXT: &str = "XMAS";
const XMAS_TEXT_REVERSED: &str = "SAMX";

fn get_location_count(data: &[&str], locations: Vec<(usize, usize)>, text_1: &str, text_2: &str) -> i32 {
    if locations.len() < text_1.len() {
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
            let xmas_char = &text_1[i..(i + 1)];
            has_text = current_char == xmas_char;
        }

        if has_text_reversed {
            let xmas_char = &text_2[i..(i + 1)];
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

    get_location_count(data, locations, XMAS_TEXT, XMAS_TEXT_REVERSED)
}

fn check_down(data: &[&str], y: usize, x: usize) -> i32 {
    let locations = (y..(y + XMAS_TEXT.len()))
        .filter(|yy| *yy < data.len())
        .map(|yy| (yy, x))
        .collect::<Vec<_>>();

    get_location_count(data, locations, XMAS_TEXT, XMAS_TEXT_REVERSED)
}

fn check_diag_right_down(data: &[&str], y: usize, x: usize) -> i32 {
    let locations = (x..(x + XMAS_TEXT.len()))
        .enumerate()
        .map(|(i, xx)| (y + i, xx))
        .filter(|(yy, xx)| *yy < data.len() && *xx < data[*yy].len())
        .collect::<Vec<_>>();

    get_location_count(data, locations, XMAS_TEXT, XMAS_TEXT_REVERSED)
}

fn check_diag_right_up(data: &[&str], y: usize, x: usize) -> i32 {
    let locations = (x..(x + XMAS_TEXT.len()))
        .enumerate()
        .map(|(i, xx)| (y.wrapping_sub(i), xx))
        .filter(|(yy, xx)| *yy < data.len() && *xx < data[*yy].len())
        .collect::<Vec<_>>();

    get_location_count(data, locations, XMAS_TEXT, XMAS_TEXT_REVERSED)
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

fn is_xmas_cross(data: &[&str], y: usize, x: usize) -> bool {
    const MAS_TEXT: &str = "MAS";
    const MAS_TEXT_REVERSED: &str = "SAM";

    // Diag right down
    let locations_1 = (0..MAS_TEXT.len())
        .map(|i| ((y - 1) + i, (x - 1) + i))
        .collect::<Vec<_>>();

    // Diag right up
    let locations_2 = (0..MAS_TEXT.len())
        .map(|i| ((y + 1) - i, (x - 1) + i))
        .collect::<Vec<_>>();

    let l1_res = get_location_count(data, locations_1, MAS_TEXT, MAS_TEXT_REVERSED);
    let l2_res = get_location_count(data, locations_2, MAS_TEXT, MAS_TEXT_REVERSED);

    l1_res > 0 && l2_res > 0
}

fn find_xmas_cross_count(data: &[&str]) -> i32 {
    let mut xmas_count = 0;

    for y in 1..(data.len() - 1) {
        for x in 1..(data[y].len() - 1) {
            if is_xmas_cross(data, y, x) {
                xmas_count += 1;
            }
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

    #[rstest]
    #[case(TEST_DATA_0, 9)]
    #[case(TEST_DATA_1, 1912)]
    fn find_xmas_cross_count_test<const N: usize>(#[case] data: [&str; N], #[case] expected: i32) {
        let actual = find_xmas_cross_count(&data);

        assert_eq!(expected, actual);
    }
}