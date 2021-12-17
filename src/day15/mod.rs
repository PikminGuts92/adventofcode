#[cfg(test)] mod data;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;

pub struct CavePath<'a, const N: usize> {
    pub pos: (usize, usize),
    pub visited: HashSet<(usize, usize)>,
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
    pub fn new(pos: (usize, usize), visited: HashSet<(usize, usize)>, risk: i16, cave: &'a [[u8; N]; N], scale: usize) -> CavePath<'a, N> {
        let (y, x) = pos;

        let acc_y = y % cave.len();
        let actual_pos = (acc_y, x % cave[acc_y].len());

        let mut cave = CavePath {
            pos,
            visited,
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

    pub fn get_up_path(&self) -> Option<CavePath<'a, N>> {
        let (acc_y, acc_x) = self.actual_pos;
        let (y, x) = self.pos;

        if acc_y == 0 {
            return None;
        }

        let pos =  (y - 1, x);
        let (y_scale, x_scale) = self.get_pos_scale(&pos);

        let next_acc_y = if acc_y - 1 == self.cave.len() {
            0
        } else {
            acc_y - 1
        };

        // Move down
        if next_acc_y < self.cave.len() && y_scale < self.scale {
            let pos_risk = (((((self.cave[next_acc_y][acc_x] as usize) + y_scale + x_scale) as i16) - 1) % 9) + 1;
            let mut new_visited = self.visited.to_owned();
            new_visited.insert(self.pos);

            Some(CavePath::new(pos, new_visited, self.risk + pos_risk, self.cave, self.scale))
        } else {
            None
        }
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
            let mut new_visited = self.visited.to_owned();
            new_visited.insert(self.pos);

            Some(CavePath::new(pos, new_visited, self.risk + pos_risk, self.cave, self.scale))
        } else {
            None
        }
    }

    pub fn get_left_path(&self) -> Option<CavePath<'a, N>> {
        let (acc_y, acc_x) = self.actual_pos;
        let (y, x) = self.pos;

        if acc_x == 0 {
            return None;
        }

        let pos =  (y, x - 1);
        let (y_scale, x_scale) = self.get_pos_scale(&pos);

        let next_acc_x = if acc_x - 1 == self.cave[acc_y].len() {
            0
        } else {
            acc_x - 1
        };

        // Move right
        if next_acc_x < self.cave[acc_y].len() && x_scale < self.scale {
            let pos_risk = (((((self.cave[acc_y][next_acc_x] as usize) + y_scale + x_scale) as i16) - 1) % 9) + 1;
            let mut new_visited = self.visited.to_owned();
            new_visited.insert(self.pos);

            Some(CavePath::new(pos, new_visited, self.risk + pos_risk, self.cave, self.scale))
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

        // Move right
        if next_acc_x < self.cave[acc_y].len() && x_scale < self.scale {
            let pos_risk = (((((self.cave[acc_y][next_acc_x] as usize) + y_scale + x_scale) as i16) - 1) % 9) + 1;
            let mut new_visited = self.visited.to_owned();
            new_visited.insert(self.pos);

            Some(CavePath::new(pos, new_visited, self.risk + pos_risk, self.cave, self.scale))
        } else {
            None
        }
    }

    pub fn get_next_paths(&self) -> Vec<CavePath<'a, N>> {
        let mut paths = Vec::new();

        if let Some(up_path) = self.get_up_path() {
            paths.push(up_path);
        }

        if let Some(down_path) = self.get_down_path() {
            paths.push(down_path);
        }

        if let Some(left_path) = self.get_left_path() {
            paths.push(left_path);
        }

        if let Some(right_path) = self.get_right_path() {
            paths.push(right_path);
        }

        // Filter out visited
        paths.retain(|p| !self.visited.contains(&p.pos));

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

    let root = CavePath::new((0, 0), HashSet::new(), 0, cave, scale);
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
    #[case(TEST_DATA_1, 5, 2957)]
    pub fn find_risk_level_test<const N: usize>(#[case] cave: [[u8; N]; N], #[case] scale: usize, #[case] expected: i16) {
        let result = find_risk_level(&cave, scale);
        assert_eq!(expected, result);
    }

    /* You can traverse up/left too
    #[rstest]
    #[case((0, 0), 0, 1, [(1, 0), (0, 1)], [4, 2])]
    #[case((0, 0), 5, 1, [(1, 0), (0, 1)], [9, 7])]
    #[case((0, 0), 6, 1, [(1, 0), (0, 1)], [10, 8])]
    #[case((0, 0), 7, 1, [(1, 0), (0, 1)], [11, 9])]
    #[case((0, 0), 8, 1, [(1, 0), (0, 1)], [12, 10])]
    #[case((0, 2), 0, 1, [(1, 2)], [6])]
    #[case((0, 2), 0, 2, [(1, 2), (0, 3)], [6, 2])]
    #[case((0, 5), 0, 2, [(1, 5)], [7])]
    #[case((2, 0), 0, 1, [(2, 1)], [8])]
    #[case((2, 0), 0, 2, [(3, 0), (2, 1)], [2, 8])]
    #[case((5, 0), 0, 2, [(5, 1)], [9])]
    #[case((2, 2), 0, 1, [], [])]
    #[case((2, 2), 0, 2, [(3, 2), (2, 3)], [4, 8])]
    #[case((2, 2), 200, 2, [(3, 2), (2, 3)], [204, 208])]
    #[case((5, 5), 0, 2, [], [])]
    #[case((8, 0), 0, 3, [(8, 1)], [1])]
    #[case((2, 6), 0, 3, [(3, 6), (2, 7)], [4, 1])]
    pub fn cave_path_get_paths_test<T: AsRef<[(usize, usize)]>, S: AsRef<[i16]>>(#[case] pos: (usize, usize), #[case] init_risk: i16, #[case] scale: usize, #[case] expected_next_path_positions: T, #[case] expected_next_path_risks: S) {
        let cave = [
            [1, 2, 3], // [2, 3, 4],
            [4, 5, 6], // [5, 6, 7],
            [7, 8, 9], // [8, 9, 1],
         // [2, 3, 4],    [3, 4, 5],
         // [5, 6, 7],    [6, 7, 8],
         // [8, 9, 1],    [9, 1, 6],
        ];

        let path = CavePath::new(pos, HashSet::new(), init_risk, &cave, scale);
        let next_paths = path.get_next_paths();

        let next_path_positions = next_paths
            .iter()
            .map(|p| p.pos)
            .collect::<Vec<_>>();

        let next_path_risks = next_paths
            .iter()
            .map(|p| p.risk)
            .collect::<Vec<_>>();

        assert_eq!(expected_next_path_positions.as_ref(), next_path_positions.as_slice());
        assert_eq!(expected_next_path_risks.as_ref(), next_path_risks.as_slice());
    }*/

    #[rstest]
    #[case((0, 0), 1, false)]
    #[case((0, 2), 1, false)]
    #[case((2, 0), 1, false)]
    #[case((2, 2), 1, true)]
    #[case((2, 2), 2, false)]
    #[case((0, 0), 2, false)]
    #[case((0, 5), 2, false)]
    #[case((5, 0), 2, false)]
    #[case((5, 5), 2, true)]
    pub fn cave_path_is_end_test(#[case] pos: (usize, usize), #[case] scale: usize, #[case] expected: bool) {
        let cave = [
            [1, 2, 3], // [2, 3, 4],
            [4, 5, 6], // [5, 6, 7],
            [7, 8, 9], // [8, 9, 1],
         // [2, 3, 4],    [3, 4, 5],
         // [5, 6, 7],    [6, 7, 8],
         // [8, 9, 1],    [9, 1, 6],
        ];

        let path = CavePath::new(pos, HashSet::new(), 0, &cave, scale);
        let result = path.is_end();

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case((0, 0), 1, (0, 0))]
    #[case((0, 2), 1, (0, 2))]
    #[case((2, 0), 1, (2, 0))]
    #[case((2, 2), 1, (2, 2))]
    #[case((2, 2), 2, (2, 2))]
    #[case((0, 0), 2, (0, 0))]
    #[case((0, 5), 2, (0, 2))]
    #[case((5, 0), 2, (2, 0))]
    #[case((5, 5), 2, (2, 2))]
    pub fn cave_path_actual_pos_test(#[case] pos: (usize, usize), #[case] scale: usize, #[case] expected: (usize, usize)) {
        let cave = [
            [1, 2, 3], // [2, 3, 4],
            [4, 5, 6], // [5, 6, 7],
            [7, 8, 9], // [8, 9, 1],
         // [2, 3, 4],    [3, 4, 5],
         // [5, 6, 7],    [6, 7, 8],
         // [8, 9, 1],    [9, 1, 6],
        ];

        let path = CavePath::new(pos, HashSet::new(), 0, &cave, scale);
        let result = path.actual_pos;

        assert_eq!(expected, result);
    }
}