#[cfg(test)] mod data;

pub fn transform_data_stacks<const N: usize>(raw_stacks: &[[Option<char>; N]]) -> Vec<Vec<char>> {
    todo!()
}

pub fn find_top_stacks_in_rearrangement(stacks: &[Vec<char>], instructions: &[(u32, u32, u32)]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_STACKS_0, TEST_DATA_INSTRUCTIONS_0, "CMZ")]
    #[case(TEST_DATA_STACKS_1, TEST_DATA_INSTRUCTIONS_1, "")]
    fn find_top_stacks_in_rearrangement_test<const N1: usize, const N2: usize, const M: usize>(
        #[case] raw_data: [[Option<char>; N1]; N2],
        #[case] instructions: [(u32, u32, u32); M],
        #[case] expected: &str) {
        let stacks = transform_data_stacks(&raw_data);
        let result = find_top_stacks_in_rearrangement(&stacks, &instructions);

        assert_eq!(expected, &result);
    }
}