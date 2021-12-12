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

pub fn traverse_path<'a>(caves: &HashMap<&'a str, Vec<&'a str>>, mut path: Vec<&'a str>, node: &'a str) -> Vec<Vec<&'a str>> {
    let mut paths = Vec::new();
    path.push(node);

    // End path found, return
    if node.eq(END) {
        paths.push(path);
        return paths;
    } else if path_contains_multiple_small_caves(&path) {
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

        let mut branch_paths = traverse_path(caves, path.to_owned(), branch);
        paths.append(&mut branch_paths);
    }

    paths
}

pub fn find_paths<'a>(segments: &'a [&'a str]) -> Vec<Vec<&'a str>> {
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

    let paths = traverse_path(&caves, Vec::new(), START);
    paths
}

pub fn find_path_count_of_small_caves(segments: &[&str]) -> i64 {
    let paths = find_paths(segments);

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
    #[case(TEST_DATA_0_1, 10)]
    #[case(TEST_DATA_0_2, 19)]
    #[case(TEST_DATA_1, 226)]
    #[case(TEST_DATA_2, 5756)]
    pub fn find_path_count_of_small_caves_test<T: AsRef<[&'static str]>>(#[case] lines: T, #[case] expected: i64) {
        let result = find_path_count_of_small_caves(lines.as_ref());
        assert_eq!(expected, result);
    }
}