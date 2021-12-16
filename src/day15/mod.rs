#[cfg(test)] mod data;

pub fn find_risk_level_nested<const N: usize>(cave: &[[u8; N]; N], (y, x): (usize, usize), mut curr_risk: i64) -> i64 {
    if (y + 1) >= cave.len() && (x + 1) >= cave[y].len() {
        return curr_risk;
    }

    // Update risk
    curr_risk += cave[y][x] as i64;

    let mut least_risk = i64::MAX;

    /*let mut moves = vec![
        // Move down
        if (y + 1) < cave.len() {
            Some(((y + 1, x), cave[y + 1][x]))
        } else {
            None
        },
        // Move right
        if (x + 1) < cave[y].len() {
            Some(((y, x + 1), cave[y][x + 1]))
        } else {
            None
        },
    ];*/

    // Move down
    if (y + 1) < cave.len() {
        let pos =  (y + 1, x);
        let risk = find_risk_level_nested(cave, pos, curr_risk);

        if risk.lt(&least_risk) {
            least_risk = risk;
        }
    }

    // Move right
    if (x + 1) < cave[y].len() {
        let pos =  (y, x + 1);
        let risk = find_risk_level_nested(cave, pos, curr_risk);

        if risk.lt(&least_risk) {
            least_risk = risk;
        }
    }

    least_risk
}

pub fn find_risk_level<const N: usize>(cave: &[[u8; N]; N]) -> i64 {
    find_risk_level_nested(cave, (0, 0), 0)
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 40)]
    #[case(TEST_DATA_1, 0)]
    pub fn find_risk_level_test<const N: usize>(#[case] cave: [[u8; N]; N], #[case] expected: i64) {
        let result = find_risk_level(&cave);
        assert_eq!(expected, result);
    }
}