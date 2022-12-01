#[cfg(test)] mod data;

use std::collections::HashMap;

pub struct Scanner<const N: usize>(Vec<[i32; N]>);

impl <const N: usize> Scanner<N> {
    pub fn from_str_multi(data: &str) -> Vec<Scanner<N>> {
        let mut scanners = Vec::new();
        let mut curr_scanner = None;

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

            let mut beacon = [0i32; N];

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

    pub fn dimensions(&self) -> usize {
        N
    }

    pub fn is_2d(&self) -> bool {
        self.dimensions().eq(&2)
    }

    pub fn is_3d(&self) -> bool {
        self.dimensions().eq(&3)
    }
}

pub fn find_beacon_count<const N: usize>(scanners: Vec<Scanner<N>>) -> i64 {
    println!("Ugh");

    0
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    const N_2D: usize = 2;
    const N_3D: usize = 3;

    #[rstest]
    #[case(TEST_DATA_0_RAW, 3)]
    pub fn find_beacon_count_2d_test(#[case] data: &str, #[case] expected: i64) {
        let scanners: Vec<Scanner<N_2D>> = Scanner::from_str_multi(data.as_ref());
        let result = find_beacon_count(scanners);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_1_RAW, 79)]
    #[case(TEST_DATA_2_RAW, 0)]
    pub fn find_beacon_count_3d_test(#[case] data: &str, #[case] expected: i64) {
        let scanners: Vec<Scanner<N_3D>> = Scanner::from_str_multi(data.as_ref());
        let result = find_beacon_count(scanners);

        assert_eq!(expected, result);
    }
}