#[cfg(test)] mod data;

pub struct Compartment {
    pub part1: [u8; 52],
    pub part2: [u8; 52],
    pub shared: Vec<char>,
    pub shared_points: u32,
}

impl Compartment {
    fn new() -> Compartment {
        Compartment {
            part1: [0u8; 52],
            part2: [0u8; 52],
            shared: Vec::new(),
            shared_points: 0u32,
        }
    }

    pub fn from_str(data: &str) -> Compartment {
        let mut comp = Compartment::new();

        // Split data in half
        let (d1, d2) = data.split_at(data.len() / 2);

        // Enumerate data in compartments
        for (data, part) in [(d1, &mut comp.part1), (d2, &mut comp.part2)] {
            for c in data.chars() {
                let index = get_index(c);
                part[index] += 1;
            }
        }

        // Find chars in both
        for i in 0..comp.part1.len() {
            let (c1, c2) = (comp.part1[i], comp.part2[i]);

            if c1 == 0 && c2 == 0 {
                continue;
            }

            // TODO: Add shared chars?

            let in_both = c1.ge(&1) && c2.ge(&1);
            if in_both {
                comp.shared_points += (i + 1) as u32;
            }
        }

        comp
    }
}

pub fn get_index(c: char) -> usize {
    match c {
        'a'..='z' => ((c as u8) - ('a' as u8)) as usize,
        'A'..='Z' => ((c as u8) - ('A' as u8) + 26) as usize,
        _ => unimplemented!("Character of '{c}' not supported")
    }
}

pub fn parse_compartments(data: &[&str]) -> Vec<Compartment> {
    data
        .iter()
        .map(|d| Compartment::from_str(d))
        .collect()
}

pub fn find_sum_of_priorities(compartments: &[Compartment]) -> u32 {
    compartments
        .iter()
        .map(|comp| comp.shared_points)
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case('a', 0)]
    #[case('b', 1)]
    #[case('c', 2)]
    #[case('x', 23)]
    #[case('y', 24)]
    #[case('z', 25)]
    #[case('A', 26)]
    #[case('B', 27)]
    #[case('C', 28)]
    #[case('X', 49)]
    #[case('Y', 50)]
    #[case('Z', 51)]
    fn get_index_test(#[case] c: char, #[case] expected: usize) {
        let result = get_index(c);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_0, 157)]
    #[case(TEST_DATA_1, 0)]
    fn find_sum_of_priorities_test<const N: usize>(#[case] raw_data: [&str; N], #[case] expected: u32) {
        let compartments = parse_compartments(&raw_data);
        let result = find_sum_of_priorities(&compartments);

        assert_eq!(expected, result);
    }
}