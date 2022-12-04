#[cfg(test)] mod data;

pub fn is_one_fully_contained(elf_1: &(u32, u32), elf_2: &(u32, u32)) -> bool {
    let ranges = [(elf_1, elf_2), (elf_2, elf_1)];

    for ((r1_min, r1_max), (r2_min, r2_max)) in ranges {
        if r1_min.ge(r2_min) && r1_max.le(r2_max) {
            return true;
        }
    }

    false
}

pub fn calc_fully_contained_counts(pairs: &[[(u32, u32); 2]]) -> usize {
    pairs
        .iter()
        .filter(|[a, b]| is_one_fully_contained(a, b))
        .count()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 2)]
    #[case(TEST_DATA_1, 475)]
    fn calc_fully_contained_counts_test<const N: usize>(#[case] data: [[(u32, u32); 2]; N], #[case] expected: usize) {
        let result = calc_fully_contained_counts(&data);

        assert_eq!(expected, result);
    }
}