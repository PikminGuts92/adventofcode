#[cfg(test)] mod data;

use std::collections::HashMap;

pub struct Scanner(Vec<[i32; 3]>);

pub fn parse_raw_data(data: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut curr_scanner: Option<Scanner> = None;

    for line in data.lines() {
        if line.starts_with("---") {
            curr_scanner = Some(Scanner(Vec::new()));
            continue;
        } else if line.is_empty() {
            // Clear curr scanner and add to collection
            if let Some(scanner) = curr_scanner.take() {
                scanners.push(scanner);
            }
            continue;
        }

        let mut beacon = [0i32; 3];

        // Parse number
        line
            .split_terminator(',')
            .enumerate()
            .for_each(|(i, s)| {
                let n = i32::from_str_radix(s, 10).unwrap();
                beacon[i] = n;
            });

        if let Some(Scanner(beacons)) = &mut curr_scanner {
            beacons.push(beacon);
        }
    }

    // Clear curr scanner and add to collection
    if let Some(scanner) = curr_scanner {
        scanners.push(scanner);
    }

    scanners
}

pub fn find_beacon_count(data: &str) -> i64 {
    let scanners = parse_raw_data(data);

    0
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1_RAW, 79)]
    #[case(TEST_DATA_2_RAW, 0)]
    pub fn find_beacon_count_test(#[case] data: &str, #[case] expected: i64) {
        let result = find_beacon_count(data.as_ref());
        assert_eq!(expected, result);
    }
}