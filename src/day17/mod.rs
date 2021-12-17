#[cfg(test)] mod data;

use std::ops::RangeInclusive;

pub fn find_max_height(boundary: &[RangeInclusive<i64>; 2], start: (i64, i64)) -> i64 {
    // x velocity: pos
    // y velocity: neg or pos

    // TODO: Evaluate from input boundary
    let x_range = 1..1000i64;
    let y_range = -1000..1000i64;

    let max_steps = 1000;

    /*let mut x_velocity = 1;
    let mut y_velocity = -200;

    let x_velocity_max = 200i64;
    let y_velocity_max = 200i64;*/

    let mut max_height = i64::MIN;

    for x_velocity in x_range {
        for y_velocity in y_range.to_owned() {
            let (mut x_pos, mut y_pos) = start;
            let mut step_x_velocity = x_velocity;
            let mut step_y_velocity = y_velocity;

            let mut in_boundary = false;
            let mut local_max_height = i64::MIN;

            for _ in 0..max_steps {
                // Check if lost cause
                let x_out_of_bounds = boundary[0].end().lt(&x_pos);
                if x_out_of_bounds || (x_out_of_bounds && boundary[1].start().lt(&y_pos)) {
                    break;
                }

                in_boundary = in_boundary || boundary[0].contains(&x_pos) && boundary[1].contains(&y_pos);
                local_max_height = local_max_height.max(y_pos);

                if step_x_velocity == 0 {
                    //break;
                }

                // Update position
                x_pos += step_x_velocity;
                y_pos += step_y_velocity;

                // Update x velocity
                step_x_velocity += if step_x_velocity > 0 {
                    -1
                } else if step_x_velocity < 0 {
                    1
                } else {
                    0
                };

                // Update y velocity
                step_y_velocity -= 1;
            }

            if in_boundary {
                // Update max height
                max_height = max_height.max(local_max_height);
            }
        }
    }


    max_height
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, (0, 0), 45)]
    #[case(TEST_DATA_2, (0, 0), 33670)]
    pub fn find_max_height_test(#[case] boundary: [RangeInclusive<i64>; 2], #[case] start: (i64, i64), #[case] expected: i64) {
        let result = find_max_height(&boundary, start);
        assert_eq!(expected, result);
    }
}