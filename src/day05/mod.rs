#[cfg(test)] mod data;

use std::collections::HashMap;

pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl From<[u32; 2]> for Point {
    fn from(p: [u32; 2]) -> Self {
        Point {
            x: p[0],
            y: p[1],
        }
    }
}

pub fn is_vertical(p1: &(u32, u32), p2: &(u32, u32)) -> bool {
    let (x1, _) = p1;
    let (x2, _) = p2;

    x1 == x2
}

pub fn is_horizontal(p1: &(u32, u32), p2: &(u32, u32)) -> bool {
    let (_, y1) = p1;
    let (_, y2) = p2;

    y1 == y2
}

pub fn is_diagonal_45((x1, y1): &(u32, u32), (x2, y2): &(u32, u32)) -> bool {
    let slope = (*y2 as i32 - *y1 as i32) / (*x2 as i32 - *x1 as i32);

    slope.abs() == 1
}

pub fn create_line_vertical((x1, y1): &(u32, u32), (_x2, y2): &(u32, u32)) -> Vec<(u32, u32)> {
    let (start, end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    let mut points = Vec::new();

    for y in *start..(*end + 1) {
        points.push((*x1, y));
    }

    points
}

pub fn create_line_horizontal((x1, y1): &(u32, u32), (x2, _y2): &(u32, u32)) -> Vec<(u32, u32)> {
    let (start, end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

    let mut points = Vec::new();

    for x in *start..(*end + 1) {
        points.push((x, *y1));
    }

    points
}

pub fn create_line_diagonal_45((x1, y1): &(u32, u32), (x2, y2): &(u32, u32)) -> Vec<(u32, u32)> {
    let slope = (*y2 as i32 - *y1 as i32) / (*x2 as i32 - *x1 as i32);
    let ((start_x, start_y), (end_x, _end_y)) = if x1 < x2 { ((x1, y1), (x2, y2)) } else { ((x2, y2), (x1, y1)) };

    let mut points = Vec::new();
    let mut y = *start_y;

    for x in *start_x..(*end_x + 1) {
        points.push((x, y));

        if y == 0 && slope == -1 {
            break;
        }

        y = match slope {
            1 => y + 1,
            _ => y - 1
        };
    }

    points
}

pub fn calculate_intersections(points: &[[[u32; 2]; 2]], diagonal: bool) -> u32 {
    let points = points
        .iter()
        .map(|p| (p[0].into(), p[1].into()))
        .collect::<Vec<(Point, Point)>>();

    let mut placements = HashMap::new();

    for (p1, p2) in points.iter() {
        let p1 = (p1.x, p1.y);
        let p2 = (p2.x, p2.y);

        let line_points;

        if is_horizontal(&p1, &p2) {
            line_points = create_line_horizontal(&p1, &p2);
        } else if is_vertical(&p1, &p2) {
            line_points = create_line_vertical(&p1, &p2);
        } else if diagonal && is_diagonal_45(&p1, &p2) {
            line_points = create_line_diagonal_45(&p1, &p2);
        } else {
            // Skip for now
            continue;
        }

        for p in line_points {
            if let Some(count) = placements.get_mut(&p) {
                *count += 1;
            } else {
                placements.insert(p, 1);
            }
        }
    }

    // Return intersection count
    placements
        .values()
        .filter(|p| p.ge(&&2))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, false, 5)]
    #[case(TEST_DATA_2, false, 6548)]
    #[case(TEST_DATA_1, true, 12)]
    #[case(TEST_DATA_2, true, 19663)]
    fn calculate_intersections_test<T: AsRef<[[[u32; 2]; 2]]>>(#[case] points: T, #[case] diagonal: bool, #[case] expected: u32) {
        let result = calculate_intersections(points.as_ref(), diagonal);
        assert_eq!(expected, result);
    }
}