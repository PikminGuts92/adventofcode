#[cfg(test)] mod data;

use std::collections::{HashMap, HashSet};

const MAX_CHARS: usize = 22; // 'A' -> 'V'

pub fn grow_polymer(template: &str, rules: &[([char; 2], char)], mut steps: u32) -> (i64, i64) {
    // Transform rules
    let mut transforms = HashMap::new();

    let rules_hashed = rules
        .iter()
        .map(|([a, b], _)| (*a, *b))
        .collect::<HashSet<_>>();

    for (seq, ins) in rules.iter() {
        let a = seq[0];
        let b = seq[1];
        let c = *ins;

        let ac = (a, c);
        let cb = (c, b);

        let b1 = rules_hashed
            .contains(&ac)
            .then_some(ac);

        let b2 = rules_hashed
            .contains(&cb)
            .then_some(cb);

        if b1.is_some() || b2.is_some() {
            transforms.insert((a, b), (b1, b2));

            /*let b1_value = b1
                .map(|(a, b)| format!("{a}{b}"))
                .unwrap_or(String::from("??"));

            let b2_value = b2
                .map(|(a, b)| format!("{a}{b}"))
                .unwrap_or(String::from("??"));

            println!("({}, {})", b1_value, b2_value);*/
        }
    }

    let mut char_counts = [0i64; MAX_CHARS];
    let mut seq_counts = [[0i64; MAX_CHARS]; MAX_CHARS];

    // Setup initial state
    {
        let mut chars_iter = template.chars();
        let mut a_char = chars_iter.next();

        while let Some(b) = chars_iter.next() {
            let a = a_char.unwrap();

            if rules_hashed.contains(&(a, b)) {
                let a = ((a as u8) - ('A' as u8)) as usize;
                let b = ((b as u8) - ('A' as u8)) as usize;

                seq_counts[a][b] += 1;
            }

            // Update
            a_char = Some(b);
        }

        // Setup initial char counts
        for c in template.chars() {
            let c = ((c as u8) - ('A' as u8)) as usize;
            char_counts[c] += 1;
        }

        /*for j in 1..template.len() {
            let i = j - 1;

            let a = template.chars().nth(i).unwrap();
            let b = template.chars().nth(j).unwrap();

            if rules_hashed.contains(&(a, b)) {
                let a = ((a as u8) - ('A' as u8)) as usize;
                let b = ((b as u8) - ('A' as u8)) as usize;

                seq_counts[a][b] += 1;
            }
        }*/
    }

    println!("Step #0");
    let letter_count: i64 = char_counts.iter().sum();
    println!("{letter_count} letters");

    let seq_count: i64 = seq_counts.iter().flatten().sum();
    println!("{seq_count} sequences\n");

    // Iterate steps
    let mut i = 1;
    while steps > 0 {
        let mut updates = Vec::new();

        for a in 0..seq_counts.len() {
            for b in 0..seq_counts[a].len() {
                let a_char = ((a as u8) + ('A' as u8)) as char;
                let b_char = ((b as u8) + ('A' as u8)) as char;

                let count = seq_counts[a][b];
                if count <= 0 {
                    continue;
                }

                if let Some((b1, b2)) = transforms.get(&(a_char, b_char)) {
                    let c = ((b1.unwrap().1 as u8) - 'A' as u8) as usize;
                    char_counts[c] += count;

                    // Remove
                    updates.push(((a_char, b_char), -count));

                    // Add
                    if let Some(b) = b1 {
                        updates.push((*b, count));
                    }

                    // Add
                    if let Some(b) = b2 {
                        updates.push((*b, count));
                    }
                } else {
                    println!("No match for {}{}", a_char, b_char);
                }
            }
        }

        // Update counts
        println!("Step #{i}");
        println!("Applying {} updates", updates.len());

        for ((a, b), count) in updates {
            let a = ((a as u8) - ('A' as u8)) as usize;
            let b = ((b as u8) - ('A' as u8)) as usize;

            seq_counts[a][b] += count;
        }

        let letter_count: i64 = char_counts.iter().sum();
        println!("{letter_count} letters");

        let seq_count: i64 = seq_counts.iter().flatten().sum();
        println!("{seq_count} sequences\n");

        steps -= 1;
        i += 1;
    }

    let min = char_counts.iter().filter(|n| n.is_positive()).min().map(|n| *n).unwrap_or_default();
    let max = char_counts.iter().max().map(|n| *n).unwrap_or_default();

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