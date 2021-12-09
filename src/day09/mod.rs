#[cfg(test)] mod data;

use std::{collections::HashSet, ops::Add};

pub fn find_heights(data: &[&str]) -> i64 {
    let data = data
        .iter()
        .map(|d| d.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let near_coords: [(i32, i32); 4] = [
        (-1,  0),
        ( 0, -1),
        ( 0,  1),
        ( 1,  0),
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

            if near.iter().all(|n| n > &c) {
                risk += (c as i64) + 1;
            }
        }
    }

    risk
}

const NEAR_COORDS: [(i64, i64); 4] = [
    (-1,  0),
    ( 0, -1),
    ( 0,  1),
    ( 1,  0),
];

pub fn traverse_basin(grid: &Vec<Vec<u32>>, y: i64, x: i64, visited: &mut HashSet<(i64, i64)>) -> i32 {
    if y.is_negative() || x.is_negative() {
        // Invalid bounds
        return 0;
    }

    let c = grid.get(y as usize).and_then(|g| g.get(x as usize));
    if c.is_none() {
        // Out of bounds
        return 0;
    }
    let c = *c.unwrap();

    if c == 9 {
        // Wall reached
        visited.insert((y, x));
        return 0;
    } else if visited.contains(&(y, x)) {
        // Already visited
        return 0;
    }

    // Update visited points
    visited.insert((y, x));

    let mut basin_size = 1;

    for (dy, dx) in NEAR_COORDS.iter() {
        basin_size += traverse_basin(grid, y + dy, x + dx, visited);
    }

    basin_size
}

pub fn find_basins(data: &[&str]) -> i64 {
    let grid = data
        .iter()
        .map(|d| d.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut basin_sizes = Vec::new();

    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if visited.contains(&(y as i64, x as i64)) {
                // Already visited
                continue;
            }

            let basin_size = traverse_basin(&grid, y as i64, x as i64, &mut visited);

            if basin_size > 1 {
                basin_sizes.push(basin_size);
            }
        }
    }

    // Sort
    basin_sizes.sort();

    // Return 3 largest sizes
    basin_sizes
        .iter()
        .rev()
        .take(3)
        .map(|s| *s as i64)
        .reduce(|acc, s| acc * s)
        .unwrap()
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

    #[rstest]
    #[case(TEST_DATA_1, 1134)]
    #[case(TEST_DATA_2, 1019700)]
    pub fn find_basins_test<T: AsRef<[&'static str]>>(#[case] data: T, #[case] expected: i64) {
        let result = find_basins(data.as_ref());
        assert_eq!(expected, result);
    }
}