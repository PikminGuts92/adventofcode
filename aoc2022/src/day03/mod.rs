#[cfg(test)] mod data;

pub struct Compartment {
    pub points: u32,
}

impl Compartment {
    fn new() -> Compartment {
        Compartment {
            points: 0u32,
        }
    }

    pub fn from_str_group<const N: usize>(data: &[&str; N]) -> Compartment {
        let mut char_counts = [[0u8; 52]; N];
        let mut comp = Compartment::new();

        // Enumerate data in compartments
        for (data, part) in data.iter().zip(&mut char_counts) {
            for c in data.chars() {
                let index = get_index(c);
                part[index] += 1;
            }
        }

        // Find chars shared between data
        for i in 0..52 {
            let all_zero = char_counts.iter().all(|c| c[i].eq(&0));
            if all_zero {
                continue;
            }

            let in_all = char_counts.iter().all(|c| c[i].ge(&1));
            if in_all {
                comp.points += (i + 1) as u32;
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

pub fn parse_compartments_in_halves(data: &[&str]) -> Vec<Compartment> {
    data
        .iter()
        .map(|d| {
            // Split data in half
            let (d1, d2) = d.split_at(d.len() / 2);

            Compartment::from_str_group(&[d1, d2])
        })
        .collect()
}

pub fn parse_compartments_in_groups_of_3(data: &[&str]) -> Vec<Compartment> {
    data
        .chunks(3)
        .map(|d| {
            Compartment::from_str_group(&[d[0], d[1], d[2]])
        })
        .collect()
}

pub fn find_sum_of_priorities(compartments: &[Compartment]) -> u32 {
    compartments
        .iter()
        .map(|comp| comp.points)
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
    #[case(TEST_DATA_1, 7746)]
    fn find_sum_of_priorities_test<const N: usize>(#[case] raw_data: [&str; N], #[case] expected: u32) {
        let compartments = parse_compartments_in_halves(&raw_data);
        let result = find_sum_of_priorities(&compartments);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_0, 70)]
    #[case(TEST_DATA_1, 2604)]
    fn find_sum_of_priorities_group_test<const N: usize>(#[case] raw_data: [&str; N], #[case] expected: u32) {
        let compartments = parse_compartments_in_groups_of_3(&raw_data);
        let result = find_sum_of_priorities(&compartments);

        assert_eq!(expected, result);
    }
}