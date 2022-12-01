#[cfg(test)] mod data;

pub fn parse_data(data: &str) -> Vec<Vec<u32>> {
    let mut parsed_data = Vec::new();
    let mut current_set = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            if !current_set.is_empty() {
                // Append set to working collection
                current_set = {
                    parsed_data.push(current_set);
                    Vec::new()
                };
            }
            continue;
        }

        let number = line.parse::<u32>().unwrap(); // Shouldn't fail
        current_set.push(number);
    }

    // Final check
    if !current_set.is_empty() {
        // Append set to working collection
        parsed_data.push(current_set);
    }

    parsed_data
}

pub fn find_most_calories(data: &Vec<Vec<u32>>) -> u64 {
    data.iter().fold(u64::MIN, |acc, row| {
        let sum = row.iter().map(|d| *d as u64).sum();
        u64::max(acc, sum)
    })
}

pub fn find_top3_calories(data: &Vec<Vec<u32>>) -> u64 {
    let mut top_calories = [u64::MIN; 3]; // Decreasing order

    for row in data.iter() {
        let row_sum: u64 = row.iter().map(|d| *d as u64).sum();

        // Get index of first element less than current sum
        let index = top_calories
            .iter()
            .enumerate()
            .filter(|(_, c)| c.lt(&&row_sum))
            .map(|(i, _)| i)
            .next();

        // Skip if no index found
        let Some(index) = index else {
            continue;
        };

        let (_, right) = top_calories.split_at_mut(index);
        right.rotate_right(1);
        right[0] = row_sum;
    }

    println!("{top_calories:?}");

    top_calories
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 24000)]
    #[case(TEST_DATA_1, 72478)]
    fn find_most_calories_test(#[case] raw_data: &str, #[case] expected: u64) {
        let data = parse_data(raw_data);

        let result = find_most_calories(&data);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_0, 45000)]
    #[case(TEST_DATA_1, 210367)]
    fn find_top3_calories_test(#[case] raw_data: &str, #[case] expected: u64) {
        let data = parse_data(raw_data);

        let result = find_top3_calories(&data);
        assert_eq!(expected, result);
    }
}