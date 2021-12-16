#[cfg(test)] mod data;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

pub struct CavePath<'a, const N: usize> {
    pub pos: (usize, usize),
    pub actual_pos: (usize, usize),
    pub risk: i16,
    pub score: f32,
    pub cave: &'a [[u8; N]; N],
    pub scale: usize,
}

impl <'a, const N: usize> fmt::Debug for CavePath<'a, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CavePath")
         .field("pos", &self.pos)
         .field("risk", &self.risk)
         .finish()
    }
}

impl <'a, const N: usize> CavePath<'a, N> {
    pub fn new(pos: (usize, usize), risk: i16, cave: &'a [[u8; N]; N], scale: usize) -> CavePath<'a, N> {
        let (y, x) = pos.to_owned();

        let acc_y = y % cave.len();
        let actual_pos = (acc_y, x % cave[acc_y].len());

        let mut cave = CavePath {
            pos,
            actual_pos,
            risk,
            score: 0.0,
            cave,
            scale
        };

        cave.score = cave.calc_distance_to_end();
        cave
    }

    pub fn is_end(&self) -> bool {
        let (y, x) = self.pos;
        let (acc_y, _) = self.actual_pos;

        (y + 1) >= (self.cave.len() * self.scale)
            && (x + 1) >= (self.cave[acc_y].len() * self.scale)
    }

    pub fn get_risk(&self) -> i16 {
        self.risk
    }

    fn get_pos_scale(&self, pos: &(usize, usize)) -> (usize, usize) {
        let (y, x) = pos;
        let acc_y = y % self.cave.len();

        ((y / self.cave.len()), (x / self.cave[acc_y].len()))
    }

    pub fn get_down_path(&self) -> Option<CavePath<'a, N>> {
        let (acc_y, acc_x) = self.actual_pos;
        let (y, x) = self.pos;

        let pos =  (y + 1, x);
        let (y_scale, x_scale) = self.get_pos_scale(&pos);

        let next_acc_y = if acc_y + 1 == self.cave.len() {
            0
        } else {
            acc_y + 1
        };

        // Move down
        if next_acc_y < self.cave.len() && y_scale < self.scale {
            let pos_risk = (((((self.cave[next_acc_y][acc_x] as usize) + y_scale + x_scale) as i16) - 1) % 9) + 1;

            Some(CavePath::new(pos, self.risk + pos_risk, self.cave, self.scale))
        } else {
            None
        }
    }

    pub fn get_right_path(&self) -> Option<CavePath<'a, N>> {
        let (acc_y, acc_x) = self.actual_pos;
        let (y, x) = self.pos;

        let pos =  (y, x + 1);
        let (y_scale, x_scale) = self.get_pos_scale(&pos);

        let next_acc_x = if acc_x + 1 == self.cave[acc_y].len() {
            0
        } else {
            acc_x + 1
        };

        //println!("({}, {})", &y_scale, &x_scale);

        // Move right
        if next_acc_x < self.cave[acc_y].len() && x_scale < self.scale {
            let pos_risk = (((((self.cave[acc_y][next_acc_x] as usize) + y_scale + x_scale) as i16) - 1) % 9) + 1;

            Some(CavePath::new(pos, self.risk + pos_risk, self.cave, self.scale))
        } else {
            None
        }
    }

    pub fn get_next_paths(&self) -> Vec<CavePath<'a, N>> {
        let mut paths = Vec::new();

        if let Some(down_path) = self.get_down_path() {
            paths.push(down_path);
        }

        if let Some(right_path) = self.get_right_path() {
            paths.push(right_path);
        }

        paths
    }

    /*pub fn expand_paths_to(&self, min_size: usize) -> Vec<CavePath<'a, N>> {
        let power = (min_size as f32).log2().ceil() as usize;

        let mut paths = Vec::new();
        self.expand_paths(&mut paths, power);

        paths
    }

    fn expand_paths(&self, paths: &mut Vec<CavePath<'a, N>>, rem: usize) {
        if rem.eq(&0) {
            return;
        } else if rem.eq(&1) {
            paths.append(&mut self.get_next_paths());
            return;
        }

        let mut branches = self.get_next_paths();

        if rem.eq(&1) {
            paths.append(&mut branches);
            return;
        }

        for branch in branches {
            branch.expand_paths(paths, rem - 1);
        }
    }*/

    fn calc_distance_to_end(&self) -> f32 {
        // Manhattan distance
        let (y, x) = self.pos;
        let (acc_y, _) = self.actual_pos;

        let (y_max, x_max) = (self.cave.len() * self.scale, self.cave[acc_y].len() * self.scale);
        ((y_max - y) + (x_max - x)) as f32
    }
}

pub fn find_risk_level<const N: usize>(cave: &[[u8; N]; N], scale: usize) -> i16 {
    if cave.len() < 1 || cave[0].len() < 1 || scale < 1 {
        return 0;
    }

    let root = CavePath::new((0, 0), 0, cave, scale);
    let mut paths = vec![root];

    // TODO: Use boxed array instead
    let mut optimal_risks = vec![vec![i16::MAX; N * scale]; N * scale];

    while !paths.is_empty() {
        // Sort paths
        paths
            .sort_by(|a, b| b.score.partial_cmp(&a.score).and_then(|o| match o {
                Ordering::Equal => a.risk.partial_cmp(&b.risk),
                _ => Some(o),
            }).unwrap());

        let path = paths.remove(0);
        let curr_risk = path.get_risk();

        let (y, x) = path.pos;
        let prev_risk = optimal_risks[y][x];

        if curr_risk.ge(&prev_risk) {
            //println!("Pruned!");
            continue;
        }

        // Update risk
        optimal_risks[y][x] = curr_risk;

        if !path.is_end() {
            // Continue path search
            let branches = path.get_next_paths();
            //branches
            //    .sort_by(|a, b| b.risk.partial_cmp(&a.risk).unwrap());

            //paths.append(&mut path.get_next_paths());
            for branch in branches {
                paths.insert(0, branch);
            }
        }
    }

    optimal_risks[(cave.len() * scale) - 1][(cave[0].len() * scale) - 1]
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 1, 40)]
    #[case(TEST_DATA_1, 1, 687)]
    #[case(TEST_DATA_0, 5, 315)]
    #[case(TEST_DATA_1, 5, 0)]
    pub fn find_risk_level_test<const N: usize>(#[case] cave: [[u8; N]; N], #[case] scale: usize, #[case] expected: i16) {
        let result = find_risk_level(&cave, scale);
        assert_eq!(expected, result);
    }
}