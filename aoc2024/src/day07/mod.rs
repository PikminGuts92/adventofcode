#[cfg(test)] mod data;

// Part 1
fn calculate_add_multiply_equation_solve_recursive(result: u64, current: u64, numbers: &[u64]) -> u64 {
    if current > result {
        return 0;
    } else if numbers.is_empty() {
        return if current == result { 1 } else { 0 };
    }

    let mut solve_count = 0;

    // Calculate add
    let add_result = current + numbers[0];
    solve_count += calculate_add_multiply_equation_solve_recursive(result, add_result, &numbers[1..]);

    // Calculate multiply
    let multiply_result = current * numbers[0];
    solve_count += calculate_add_multiply_equation_solve_recursive(result, multiply_result, &numbers[1..]);

    solve_count
}

fn calculate_add_multiply_equation_solve_count(result: u64, numbers: &[u64]) -> u64 {
    calculate_add_multiply_equation_solve_recursive(result, numbers[0], &numbers[1..])
}

fn calculate_sum_of_solvable_add_multiply_equations(data: &[(u64, &[u64])]) -> u64 {
    data
        .iter()
        .filter(|(res, nums)| calculate_add_multiply_equation_solve_count(*res, nums) > 0)
        .map(|(res, _)| *res)
        .sum()
}

// Part 2
fn calculate_add_multiply_concat_equation_solve_recursive(result: u64, current: u64, numbers: &[u64]) -> u64 {
    if current > result {
        return 0;
    } else if numbers.is_empty() {
        return if current == result { 1 } else { 0 };
    }

    let mut solve_count = 0;

    // Calculate add
    let add_result = current + numbers[0];
    solve_count += calculate_add_multiply_concat_equation_solve_recursive(result, add_result, &numbers[1..]);

    // Calculate multiply
    let multiply_result = current * numbers[0];
    solve_count += calculate_add_multiply_concat_equation_solve_recursive(result, multiply_result, &numbers[1..]);

    // Calculate concat (hacky way w/ strings)
    //let mut digits = Vec::new();
    let concat_result = format!("{}{}", current, numbers[0]).parse::<u64>().unwrap();
    solve_count += calculate_add_multiply_concat_equation_solve_recursive(result, concat_result, &numbers[1..]);

    solve_count
}

fn calculate_add_multiply_concat_equation_solve_count(result: u64, numbers: &[u64]) -> u64 {
    calculate_add_multiply_concat_equation_solve_recursive(result, numbers[0], &numbers[1..])
}

fn calculate_sum_of_solvable_add_multiply_concat_equations(data: &[(u64, &[u64])]) -> u64 {
    data
        .iter()
        .filter(|(res, nums)| calculate_add_multiply_concat_equation_solve_count(*res, nums) > 0)
        .map(|(res, _)| *res)
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 3749)]
    #[case(TEST_DATA_1, 303766880536)]
    fn calculate_sum_of_solvable_add_multiply_equations_test<const N: usize>(#[case] data: [(u64, &[u64]); N], #[case] expected: u64) {
        let actual = calculate_sum_of_solvable_add_multiply_equations(&data);

        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case(TEST_DATA_0, 11387)]
    #[case(TEST_DATA_1, 337041851384440)]
    fn calculate_sum_of_solvable_add_multiply_concat_equations_test<const N: usize>(#[case] data: [(u64, &[u64]); N], #[case] expected: u64) {
        let actual = calculate_sum_of_solvable_add_multiply_concat_equations(&data);

        assert_eq!(expected, actual);
    }
}