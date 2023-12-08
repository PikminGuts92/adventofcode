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

fn find_steps_to_end_2(data_nav: &[u8], data_nodes: &[(&[u8; 3], &[u8; 3], &[u8; 3])]) -> u32 {
    let map_index = data_nodes
        .iter()
        .enumerate()
        .map(|(i, (n, ..))| (*n, i))
        .collect::<HashMap<_, _>>();

    let (all_starts, all_ends) = data_nodes
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
        });

    let mut current_step = 0;
    let mut current_pos = 0;
    let mut current_nodes = data_nodes
        .iter()
        .enumerate()
        .filter(|(i, _)| all_starts.contains(&i))
        .map(|(_, d)| d)
        .collect::<Vec<_>>();

    loop {
        let is_end = current_nodes
            .iter()
            .all(|([_, _, c], ..)| c.eq(&b'Z'));

        if is_end {
            break;
        }

        let check_left = data_nav[current_pos].eq(&b'L');

        // Update nodes
        if check_left {
            for current_node in current_nodes.iter_mut() {
                let (_, left_node, _) = current_node;
                let left_idx = map_index.get(left_node).map(|s| *s).unwrap();

                *current_node = &data_nodes[left_idx];
            }
        } else {
            for current_node in current_nodes.iter_mut() {
                let (_, _, right_node) = current_node;
                let right_idx = map_index.get(right_node).map(|s| *s).unwrap();

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

    current_step
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
    #[case(TEST_DATA_2_NAV, TEST_DATA_2_NODES, 0)]
    fn find_steps_to_end_2_test<const N: usize>(#[case] data_nav: &[u8], #[case] data_nodes: [(&[u8; 3], &[u8; 3], &[u8; 3]); N], #[case] expected: u32) {
        let actual = find_steps_to_end_2(data_nav, &data_nodes);

        assert_eq!(expected, actual);
    }
}