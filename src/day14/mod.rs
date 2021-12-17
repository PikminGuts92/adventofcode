#[cfg(test)] mod data;

use std::collections::HashMap;

pub fn grow_polymer(template: &str, rules: &[(&str, &str)], mut steps: u32) -> String {
    let mut new_template = template.to_owned();

    while steps > 0 {
        let mut inserts = Vec::new();

        // Find sequence + track insert replacements
        for (seq, rep) in rules.iter() {
            let mut instances = Vec::new();

            for j in 1..new_template.len() {
                let i = j - 1;

                let a = new_template.get(i..(i + 1)).unwrap();
                let b = new_template.get(j..(j + 1)).unwrap();

                if seq[0..1].eq(a) && seq[1..2].eq(b) {
                    instances.push((j, rep));
                }
            }

            if !instances.is_empty() {
                inserts.append(&mut instances);
            }
        }

        let inserts_map = inserts
            .into_iter()
            .collect::<HashMap<_, _>>();

        // Build new string
        let mut chars = Vec::new();
        for (i, c) in new_template.chars().enumerate() {
            if let Some(rep) = inserts_map.get(&i) {
                // Super hacky way to get char
                chars.push(rep.as_bytes()[0] as char);
            }

            chars.push(c);
        }

        // Assign string
        new_template = chars.iter().collect();

        steps -= 1;
    }

    new_template
}

pub fn grow_polymer_get_number(template: &str, rules: &[(&str, &str)], steps: u32) -> i64 {
    let result = grow_polymer(template, rules, steps);

    let mut counts = result
        .chars()
        .into_iter()
        .fold(HashMap::new(), |mut acc, c| {
            match acc.get_mut(&c) {
                Some(count) => *count += 1,
                _ => {
                    acc.insert(c, 1);
                },
            };

            acc
        })
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect::<Vec<_>>();

    // Order counts
    counts.sort_by(|(_, a), (_, b)| a.cmp(b));

    let (_, least) = counts.first().unwrap();
    let (_, most) = counts.last().unwrap();

    *most - *least
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 1, "NCNBCHB")]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 2, "NBCCNBBBCBHCB")]
    #[case("NBCCNBBBCBHCB", TEST_DATA_1_1, 1, "NBBBCNCCNBBNBNBBCHBHHBCHB")]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 3, "NBBBCNCCNBBNBNBBCHBHHBCHB")]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 4, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")]
    pub fn grow_polymer_test<T: AsRef<str>, S: AsRef<[(&'static str, &'static str)]>>(#[case] template: T, #[case] rules: S, #[case] steps: u32, #[case] expected: &str) {
        let result = grow_polymer(template.as_ref(), rules.as_ref(), steps);
        assert_eq!(expected, &result);
    }

    #[rstest]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 10, 1588)]
    #[case(TEST_DATA_2_0, TEST_DATA_2_1, 10, 2068)]
    #[case(TEST_DATA_1_0, TEST_DATA_1_1, 40, 2188189693529)]
    #[case(TEST_DATA_2_0, TEST_DATA_2_1, 40, 0)]
    pub fn grow_polymer_get_number_test<T: AsRef<str>, S: AsRef<[(&'static str, &'static str)]>>(#[case] template: T, #[case] rules: S, #[case] steps: u32, #[case] expected: i64) {
        let result = grow_polymer_get_number(template.as_ref(), rules.as_ref(), steps);
        assert_eq!(expected, result);
    }
}