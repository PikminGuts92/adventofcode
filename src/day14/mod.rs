#[cfg(test)] mod data;

use std::collections::{HashMap, HashSet};

const MAX_CHARS: usize = 22; // 'A' -> 'V'

pub fn grow_polymer(template: &str, rules: &[([char; 2], char)], mut steps: u32) -> (i64, i64) {
    // Transform rules
    let rules = rules
        .iter()
        .map(|([a, b], c)| {
            let a = (*a as u8) - ('A' as u8);
            let b = (*b as u8) - ('A' as u8);
            let c = (*c as u8) - ('A' as u8);

            ((a, b), [(a, c), (c, b)])
        })
        .collect::<HashMap<_, _>>();

    let mut char_counts = [0i64; MAX_CHARS];
    let mut seq_counts = [[0i64; MAX_CHARS]; MAX_CHARS];

    // Setup initial state
    {
        let mut chars_iter = template.chars();
        let mut prev_char = chars_iter.next();

        while let Some(b_char) = chars_iter.next() {
            let a_char = prev_char.unwrap();

            let a = (a_char as u8) - ('A' as u8);
            let b = (b_char as u8) - ('A' as u8);

            if rules.contains_key(&(a, b)) {
                seq_counts[a as usize][b as usize] += 1;
            }

            // Update
            prev_char = Some(b_char);
        }

        // Setup initial char counts
        for c in template.chars() {
            let c = ((c as u8) - ('A' as u8)) as usize;
            char_counts[c] += 1;
        }
    }

    // Iterate steps
    while steps > 0 {
        let mut updates = Vec::new();

        for ((sa, sb), [(a, c), (_, b)]) in rules.iter() {
            let seq_count = seq_counts[*sa as usize][*sb as usize];
            if seq_count <= 0 {
                continue;
            }

            char_counts[*c as usize] += seq_count;

            updates.push(((*a, *b), -seq_count));
            updates.push(((*a, *c), seq_count));
            updates.push(((*c, *b), seq_count));
        }

        // Update counts
        for ((a, b), count) in updates {
            seq_counts[a as usize][b as usize] += count;
        }

        steps -= 1;
    }

    // Find min/max character
    let (min, max) = char_counts
        .iter()
        .fold((i64::MAX, i64::MIN), |(min, max), v: &i64 | {
            if v > &0 {
                (min.min(*v), max.max(*v))
            } else {
                (min, max)
            }
        });
    
    (min, max)
}

pub fn grow_polymer_get_number(template: &str, rules: &[([char; 2], char)], steps: u32) -> i64 {
    let (min, max) = grow_polymer(template, rules, steps);

    println!("Min: {min}, Max: {max}");
    max - min
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    /*#[rstest]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 1, "NCNBCHB")]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 2, "NBCCNBBBCBHCB")]
    #[case("NBCCNBBBCBHCB", TEST_DATA_1_1, 1, "NBBBCNCCNBBNBNBBCHBHHBCHB")]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 3, "NBBBCNCCNBBNBNBBCHBHHBCHB")]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 4, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")]
    pub fn grow_polymer_test<T: AsRef<str>, S: AsRef<[(&'static str, &'static str)]>>(#[case] template: T, #[case] rules: S, #[case] steps: u32, #[case] expected: &str) {
        let result = grow_polymer(template.as_ref(), rules.as_ref(), steps);
        assert_eq!(expected, &result);
    }*/

    #[rstest]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 10, 1588)]
    #[case(TEST_DATA_2_0, TEST_DATA_2_1, 10, 2068)]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 40, 2188189693529)]
    #[case(TEST_DATA_2_0, TEST_DATA_2_1, 40, 2158894777814)]
    pub fn grow_polymer_get_number_test<T: AsRef<str>, S: AsRef<[([char; 2], char)]>>(#[case] template: T, #[case] rules: S, #[case] steps: u32, #[case] expected: i64) {
        let result = grow_polymer_get_number(template.as_ref(), rules.as_ref(), steps);
        assert_eq!(expected, result);
    }
}