#[cfg(test)] mod data;

use std::collections::HashMap;

const START: &str = "start";
const END: &str = "end";

pub fn path_contains_multiple_small_caves<'a>(path: &Vec<&'a str>) -> bool {
    path
        .iter()
        .filter(|p| p.ne(&&START)
            && p.ne(&&END)
            && p.chars().all(|c| c.is_lowercase()))
        .fold(HashMap::new(), |mut acc, p| {
            if let Some(count) = acc.get_mut(p) {
                *count += 1;
            } else {
                acc.insert(p, 1);
            }

            acc
        })
        .values()
        .any(|v| v.gt(&1))
}

pub fn path_contains_more_than_one_small_cave_twice<'a>(path: &Vec<&'a str>) -> bool {
    let counts = path
        .iter()
        .filter(|p| p.ne(&&START)
            && p.ne(&&END)
            && p.chars().all(|c| c.is_lowercase()))
        .fold(HashMap::new(), |mut acc, p| {
            if let Some(count) = acc.get_mut(p) {
                *count += 1;
            } else {
                acc.insert(p, 1);
            }

            acc
        })
        .into_values()
        .collect::<Vec<_>>();

    if counts.iter().any(|c| c.gt(&2)) {
        return true;
    } else if counts.iter().filter(|c| c.eq(&&2)).count().gt(&1) {
        return true;
    } else {
        return false;
    }
}

pub fn traverse_path<'a>(caves: &HashMap<&'a str, Vec<&'a str>>, mut path: Vec<&'a str>, node: &'a str, max_small: u32) -> Vec<Vec<&'a str>> {
    let mut paths = Vec::new();
    path.push(node);

    // End path found, return
    if node.eq(END) {
        paths.push(path);
        return paths;
    } else if max_small == 1 && path_contains_multiple_small_caves(&path) {
        return paths;
    } else if max_small == 2 && path_contains_more_than_one_small_cave_twice(&path) {
        return paths;
    }

    let branches = match caves.get(node) {
        Some(b) => b,
        None => return paths, 
    };

    for branch in branches.iter() {
        if branch.eq(&START) {
            continue;
        }

        let mut branch_paths = traverse_path(caves, path.to_owned(), branch, max_small);
        paths.append(&mut branch_paths);
    }

    paths
}

pub fn find_paths<'a>(segments: &'a [&'a str], max_small: u32) -> Vec<Vec<&'a str>> {
    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();

    for i in 0..segments.len() {
        let split = segments[i].split("-").collect::<Vec<_>>();
        let a = split[0];
        let b = split[1];

        // Insert direction 1
        if let Some(node) = caves.get_mut(a) {
            node.push(b);
        } else {
            let mut node = Vec::new();
            node.push(b);

            caves.insert(a, node);
        }

        // Insert direction 2
        if let Some(node) = caves.get_mut(b) {
            node.push(a);
        } else {
            let mut node = Vec::new();
            node.push(a);

            caves.insert(b, node);
        }
    }

    let paths = traverse_path(&caves, Vec::new(), START, max_small);
    paths
}

pub fn find_path_count_of_small_caves(segments: &[&str], max_small: u32) -> i64 {
    let paths = find_paths(segments, max_small);

    /*for path in paths.iter() {
        println!("{path:?}");
    }*/

    paths.len() as i64
}


#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0_1, 1, 10)]
    #[case(TEST_DATA_0_2, 1, 19)]
    #[case(TEST_DATA_1, 1, 226)]
    #[case(TEST_DATA_2, 1, 5756)]
    #[case(TEST_DATA_0_1, 2, 36)]
    #[case(TEST_DATA_0_2, 2, 103)]
    #[case(TEST_DATA_1, 2, 3509)]
    #[case(TEST_DATA_2, 2, 144603)]
    pub fn find_path_count_of_small_caves_test<T: AsRef<[&'static str]>>(#[case] segments: T, #[case] max_small: u32, #[case] expected: i64) {
        let result = find_path_count_of_small_caves(segments.as_ref(), max_small);
        assert_eq!(expected, result);
    }
}