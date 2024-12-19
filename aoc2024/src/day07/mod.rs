#[cfg(test)] mod data;

/*enum Operation {
    Add,
    Multiply,
}*/

fn calculate_equation_solve_recursive(result: u64, current: u64, numbers: &[u64]) -> u64 {
    if current > result {
        return 0;
    } else if numbers.is_empty() {
        return if current == result { 1 } else { 0 };
    }

    let mut solve_count = 0;

    // Calculate add
    let add_result = current + numbers[0];
    solve_count += calculate_equation_solve_recursive(result, add_result, &numbers[1..]);

    // Calculate multiply
    let multiply_result = current * numbers[0];
    solve_count += calculate_equation_solve_recursive(result, multiply_result, &numbers[1..]);

    solve_count
}

fn calculate_equation_solve_count(result: u64, numbers: &[u64]) -> u64 {
    calculate_equation_solve_recursive(result, numbers[0], &numbers[1..])
}

fn calculate_sum_of_solvable_equations(data: &[(u64, &[u64])]) -> u64 {
    data
        .iter()
        .filter(|(res, nums)| calculate_equation_solve_count(*res, nums) > 0)
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
    fn calculate_sum_of_solvable_equations_test<const N: usize>(#[case] data: [(u64, &[u64]); N], #[case] expected: u64) {
        let actual = calculate_sum_of_solvable_equations(&data);

        assert_eq!(expected, actual);
    }
}