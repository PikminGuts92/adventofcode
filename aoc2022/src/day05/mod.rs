#[cfg(test)] mod data;

pub fn transform_data_stacks<const N: usize>(raw_stacks: &[[Option<char>; N]]) -> Vec<Vec<char>> {
    let stack_count = N;
    let mut stacks = vec![Vec::new(); stack_count];

    if raw_stacks.is_empty() {
        return stacks;
    }

    for j in 0..stack_count {
        let stack = &mut stacks[j];
        let mut i = raw_stacks.len() - 1;

        while let Some(Some(item)) = raw_stacks[i].get(j) {
            stack.push(*item);

            if i.lt(&1) {
                break;
            }

            i -= 1;
        }
    }

    stacks
}

pub fn get_top_crates(stacks: &[Vec<char>]) -> String {
    String::from_iter(stacks
        .iter()
        .map(|s| s.last().unwrap_or(&'\0')))
}

pub fn find_top_stacks_in_rearrangement(stacks: &mut [Vec<char>], instructions: &[(u32, u32, u32)]) -> String {
    for (count, from_stack, to_stack) in instructions {
        let from_idx = (from_stack - 1) as usize;
        let to_idx = (to_stack - 1) as usize;

        for _ in 0..*count {
            let item = stacks[from_idx].pop().unwrap();
            stacks[to_idx].push(item);
        }
    }

    get_top_crates(stacks)
}

pub fn find_top_stacks_in_rearrangement_retain_order(stacks: &mut [Vec<char>], instructions: &[(u32, u32, u32)]) -> String {
    for (count, from_stack, to_stack) in instructions {
        let from_idx = (from_stack - 1) as usize;
        let to_idx = (to_stack - 1) as usize;

        let mut items = Vec::new();

        for _ in 0..*count {
            let item = stacks[from_idx].pop().unwrap();
            items.push(item);
        }

        while let Some(item) = items.pop() {
            stacks[to_idx].push(item);
        }
    }

    get_top_crates(stacks)
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_STACKS_0, TEST_DATA_INSTRUCTIONS_0, "CMZ")]
    #[case(TEST_DATA_STACKS_1, TEST_DATA_INSTRUCTIONS_1, "LBLVVTVLP")]
    fn find_top_stacks_in_rearrangement_test<const N1: usize, const N2: usize, const M: usize>(
        #[case] raw_data: [[Option<char>; N1]; N2],
        #[case] instructions: [(u32, u32, u32); M],
        #[case] expected: &str) {
        let mut stacks = transform_data_stacks(&raw_data);
        let result = find_top_stacks_in_rearrangement(&mut stacks, &instructions);

        assert_eq!(expected, &result);
    }

    #[rstest]
    #[case(TEST_DATA_STACKS_0, TEST_DATA_INSTRUCTIONS_0, "MCD")]
    #[case(TEST_DATA_STACKS_1, TEST_DATA_INSTRUCTIONS_1, "TPFFBDRJD")]
    fn find_top_stacks_in_rearrangement_retain_order_test<const N1: usize, const N2: usize, const M: usize>(
        #[case] raw_data: [[Option<char>; N1]; N2],
        #[case] instructions: [(u32, u32, u32); M],
        #[case] expected: &str) {
        let mut stacks = transform_data_stacks(&raw_data);
        let result = find_top_stacks_in_rearrangement_retain_order(&mut stacks, &instructions);

        assert_eq!(expected, &result);
    }
}