#[cfg(test)] mod data;

use std::collections::HashMap;

pub fn find_heights(data: &[&str]) -> i64 {
    let data = data
        .iter()
        .map(|d| d.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let near_coords: [(i32, i32); 4] = [
        //[-1, -1],
        (-1,  0),
        //[-1,  1],
        ( 0, -1),
        //[ 0,  0],
        ( 0,  1),
        //[ 1, -1],
        ( 1,  0),
        //[ 1,  1],
    ];

    let mut risk = 0;

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            let c = data[y][x].to_digit(10).unwrap() as u32;

            let near = near_coords
                .iter()
                .filter(|(n_y, n_x)| n_y.checked_add(y as i32).is_some()
                    && n_x.checked_add(x as i32).is_some())
                .map(|(n_y, n_x)| data.get(((y as i32) + n_y) as usize)
                    .and_then(|row| row.get(((x as i32) + n_x) as usize)))
                .filter(|n| n.is_some())
                .map(|n| n.unwrap().to_digit(10).unwrap())
                .collect::<Vec<_>>();

            //println!("Found {} near!", near.len());
            //println!("{:?}", near);

            if near.iter().all(|n| n > &c) {
                risk += (c as i64) + 1;
            }
        }
    }

    risk
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 15)]
    #[case(TEST_DATA_2, 575)]
    pub fn find_heights_test<T: AsRef<[&'static str]>>(#[case] data: T, #[case] expected: i64) {
        let result = find_heights(data.as_ref());
        assert_eq!(expected, result);
    }
}