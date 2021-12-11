#[cfg(test)] mod data;

use std::collections::HashSet;

pub struct Octopus {
    pub energy: u32,
    pub discharged: bool,
}

impl Octopus {
    pub fn new(energy: u32) -> Octopus {
        Octopus {
            energy,
            discharged: false
        }
    }

    pub fn increase_level(&mut self) {
        if !self.discharged && self.energy < 10 {
            self.energy += 1;
        }
    }

    pub fn can_flash(&self) -> bool {
        !self.discharged && self.energy > 9
    }

    pub fn flash(&mut self) {
        if self.can_flash() {
            // Reset energy and discharge
            self.energy = 0;
            self.discharged = true;
        }
    }

    pub fn reset_charge(&mut self) {
        self.discharged = false;
    }
}

impl From<u32> for Octopus {
    fn from(energy: u32) -> Self {
        Octopus::new(energy)
    }
}

pub fn get_adjacent_points((y, x): (usize, usize), y_size: usize, x_size: usize) -> Vec<(usize, usize)> {
    let y_size = y_size as i64;
    let x_size = x_size as i64;

    vec![
        (-1, -1), (-1,  0), (-1,  1),
        ( 0, -1),           ( 0,  1),
        ( 1, -1), ( 1,  0), ( 1,  1)
    ]
        .iter()
        .map(|(dy, dx)| ((y as i64) + dy, (x as i64) + dx))
        .filter(|(y, x)| !y.is_negative() && y < &y_size
            && !x.is_negative() && x < &x_size)
        .map(|(y, x)| (y as usize, x as usize))
        .collect()
}

//pub fn process_octopus((y, x): (usize, usize), grid: )

pub fn count_flashes(data: &[&str], steps: u32) -> i64 {
    // Convert to octopuses
    let mut grid = data
        .iter()
        .map(|d| d.chars()
            .map(|c| c.to_digit(10).unwrap().into())
            .collect::<Vec<Octopus>>())
        .collect::<Vec<_>>();

    let y_size = grid.len();
    let x_size = grid
        .first()
        .and_then(|f| Some(f.len()))
        .unwrap_or_default();

    let mut flash_count = 0;

    for step in 0..steps {
        // Reset charge states + increase levels
        grid
            .iter_mut()
            .flatten()
            .for_each(|o| {
                o.reset_charge();
                o.increase_level();
            });

        // Check charge levels and discharge if needed
        loop {
            let mut all_safe = true;

            for y in 0..grid.len() {
                for x in 0..data[y].len() {
                    let discharged = {
                        let octopus = &mut grid[y][x];
                        if octopus.can_flash() {
                            octopus.flash();
                            flash_count += 1;
                            true
                        } else {
                            false
                        }
                    };

                    // Process flashes
                    if discharged {
                        all_safe = false;

                        let mut adjacent = get_adjacent_points((y, x), y_size, x_size);

                        while adjacent.len() > 0 {
                            let (ad_y, ad_x) = adjacent.remove(0);
                            let octopus = &mut grid[ad_y][ad_x];

                            octopus.increase_level();

                            if octopus.can_flash() {
                                octopus.flash();
                                flash_count += 1;

                                let mut more_ads = get_adjacent_points((ad_y, ad_x), y_size, x_size);
                                adjacent.append(&mut more_ads);
                            }
                        }
                    }
                }
            }

            if all_safe {
                // No more octopuses to check
                break;
            }
        }
    }

    flash_count
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 1, 9)]
    #[case(TEST_DATA_1, 2, 35)]
    #[case(TEST_DATA_1, 10, 204)]
    #[case(TEST_DATA_1, 100, 1656)]
    #[case(TEST_DATA_2, 100, 1620)]
    pub fn count_flashes_test<T: AsRef<[&'static str]>>(#[case] data: T, #[case] steps: u32, #[case] expected: i64) {
        let result = count_flashes(data.as_ref(), steps);
        assert_eq!(expected, result);
    }
}