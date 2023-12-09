use std::collections::{HashMap, HashSet};

#[cfg(test)] mod data;

fn find_steps_to_end(data_nav: &[u8], data_nodes: &[(&[u8; 3], &[u8; 3], &[u8; 3])]) -> u32 {
    let map_index = data_nodes
        .iter()
        .enumerate()
        .map(|(i, (n, ..))| (*n, i))
        .collect::<HashMap<_, _>>();

    let start = map_index.get(b"AAA").map(|s| *s).unwrap();

    let mut current_step = 0;
    let mut current_pos = 0;
    let mut current_node = &data_nodes[start];

    loop {
        let is_end = match current_node {
            (b"ZZZ", ..) => true,
            _ => false,
        };

        if is_end {
            break;
        }

        let check_left = data_nav[current_pos].eq(&b'L');

        // Update node
        current_node = if check_left {
            let (_, left_node, _) = current_node;
            let left_idx = map_index.get(left_node).map(|s| *s).unwrap();

            &data_nodes[left_idx]
        } else {
            let (_, _, right_node) = current_node;
            let right_idx = map_index.get(right_node).map(|s| *s).unwrap();

            &data_nodes[right_idx]
        };

        // Update steps
        current_step += 1;
        current_pos = if (data_nav.len() - 1) == current_pos {
            0
        } else {
            current_pos + 1
        };
    }

    current_step
}

fn find_steps_to_end_2(data_nav: &[u8], data_nodes: &[(&[u8; 3], &[u8; 3], &[u8; 3])]) -> u64 {
    let map_index = data_nodes
        .iter()
        .enumerate()
        .map(|(i, (n, ..))| (*n, i))
        .collect::<HashMap<_, _>>();

    /*let (all_starts, all_ends) = data_nodes
        .iter()
        .enumerate()
        .fold((HashSet::new(), HashSet::new()), |(mut starts, mut ends), (i, (n, ..))| {
            match n {
                [_, _, b'A'] => {
                    starts.insert(i);
                },
                [_, _, b'Z'] => {
                    ends.insert(i);
                },
                _ => {}
            }

            (starts, ends)
        });*/

    let mut current_step = 0;
    let mut current_pos = 0;
    let mut current_nodes = data_nodes
        .iter()
        .enumerate()
        //.filter(|(i, _)| all_starts.contains(&i))
        .filter(|(i, ([_, _, c], ..))| c.eq(&b'A'))
        .map(|(_, d)| d)
        .collect::<Vec<_>>();

    let mut visited_nodes = current_nodes
        .iter()
        .map(|_| HashMap::new())
        .collect::<Vec<_>>();

    loop {
        let all_visited = current_nodes
            .iter()
            .enumerate()
            .all(|(i, (n, ..))| visited_nodes[i].contains_key(&(*n, current_pos)));

        if all_visited {
            break;
        }

        let check_left = data_nav[current_pos].eq(&b'L');

        // Update nodes
        if check_left {
            for (current_node, visited) in current_nodes.iter_mut().zip(visited_nodes.iter_mut()) {
                let (n, left_node, _) = current_node;
                let left_idx = map_index.get(left_node).map(|s| *s).unwrap();

                if visited.contains_key(&(*n, current_pos)) {
                    //println!("Cycle detected (Pos: {current_pos}, Step: {current_step})");
                } else {
                    visited.insert((*n, current_pos), current_step);
                }

                *current_node = &data_nodes[left_idx];
            }
        } else {
            for (current_node, visited) in current_nodes.iter_mut().zip(visited_nodes.iter_mut()) {
                let (n, _, right_node) = current_node;
                let right_idx = map_index.get(right_node).map(|s| *s).unwrap();

                if visited.contains_key(&(*n, current_pos)) {
                    //println!("Cycle detected (Pos: {current_pos}, Step: {current_step})");
                } else {
                    visited.insert((*n, current_pos), current_step);
                }

                *current_node = &data_nodes[right_idx];
            }
        };

        // Update steps
        current_step += 1;
        current_pos = if (data_nav.len() - 1) == current_pos {
            0
        } else {
            current_pos + 1
        };
    }

    // Remove all non-ending nodes
    visited_nodes
        .iter_mut()
        .for_each(|nodes| {
            let non_z_keys = nodes
                .iter()
                .filter(|(([_, _, c], ..), _)| c.ne(&b'Z'))
                .map(|(k, v)| *k)
                .collect::<Vec<_>>();

            for key in non_z_keys {
                nodes.remove(&key);
            }
        });

    // Get lowest distinct prime numbers and multiply together
    visited_nodes
        .into_iter()
        .map(|map| {
            let min_number = map.into_values().min().unwrap() as u64;
            get_prime_factors(min_number)
        })
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .product::<u64>()
}


fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    (2..n)
        .into_iter()
        .find_map(|a| if n % a == 0 {
            Some(false)
        } else {
            None
        })
        .unwrap_or(true)
}

// Definitely not the most efficient
fn get_prime_factors(n: u64) -> Vec<u64> {
    if n == 1 {
        return Vec::new();
    }

    (2..=n)
        .into_iter()
        //.filter(|&x| n % x == 0)
        .filter(|a| n % a == 0 && is_prime(*a))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0_NAV, TEST_DATA_0_NODES, 2)]
    #[case(TEST_DATA_1_NAV, TEST_DATA_1_NODES, 6)]
    #[case(TEST_DATA_2_NAV, TEST_DATA_2_NODES, 13207)]
    fn find_steps_to_end_test<const N: usize>(#[case] data_nav: &[u8], #[case] data_nodes: [(&[u8; 3], &[u8; 3], &[u8; 3]); N], #[case] expected: u32) {
        let actual = find_steps_to_end(data_nav, &data_nodes);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_3_NAV, TEST_DATA_3_NODES, 6)]
    #[case(TEST_DATA_2_NAV, TEST_DATA_2_NODES, 12_324_145_107_121)]
    fn find_steps_to_end_2_test<const N: usize>(#[case] data_nav: &[u8], #[case] data_nodes: [(&[u8; 3], &[u8; 3], &[u8; 3]); N], #[case] expected: u64) {
        let actual = find_steps_to_end_2(data_nav, &data_nodes);

        assert_eq!(expected, actual);
    }
}