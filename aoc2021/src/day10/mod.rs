#[cfg(test)] mod data;

pub fn find_invalid_chars(chars: &Vec<char>, i: usize, open_stack: &mut Vec<char>, invalid: &mut Vec<char>) -> bool {
    if i >= chars.len() {
        return open_stack.is_empty();
    }

    let open = open_stack.pop();
    let curr = chars[i];

    // Check if valid
    let is_valid = match (open, curr) {
        (Some('('), ')') => true,
        (Some('['), ']') => true,
        (Some('{'), '}') => true,
        (Some('<'), '>') => true,
        (_, '(') => true,
        (_, '[') => true,
        (_, '{') => true,
        (_, '<') => true,
        _ => false,
    };

    if !is_valid {
        // Fast exit for now
        invalid.push(curr);
        return false;
    }

    let is_closed = match (open, curr) {
        (Some('('), ')') => true,
        (Some('['), ']') => true,
        (Some('{'), '}') => true,
        (Some('<'), '>') => true,
        _ => false,
    };

    if !is_closed {
        match open {
            Some('(' | '[' | '{' | '<') => open_stack.push(open.unwrap()),
            _ => {}
        }

        match curr {
            '(' | '[' | '{' | '<' => open_stack.push(curr),
            _ => {}
        }
    }

    find_invalid_chars(chars, i + 1, open_stack, invalid)
}

pub fn find_syntax_errors_in_line(line: &str) -> i64 {
    if line.is_empty() {
        return 0;
    }

    let chars = line.chars().collect::<Vec<_>>();

    let mut open_stack = Vec::new();
    let mut invalid_chars = Vec::new();
    let complete = find_invalid_chars(&chars, 0, &mut open_stack, &mut invalid_chars);

    if invalid_chars.is_empty() || complete {
        return 0;
    }

    // Calculate score
    let score = invalid_chars
        .iter()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum();

    score
}

pub fn find_syntax_errors(lines: &[&str]) -> i64 {
    lines
        .iter()
        .map(|l| find_syntax_errors_in_line(l))
        .sum()
}

pub fn find_incomplete_in_line(line: &str) -> i64 {
    if line.is_empty() {
        return 0;
    }

    let chars = line.chars().collect::<Vec<_>>();

    let mut open_stack = Vec::new();
    let mut invalid_chars = Vec::new();
    let complete = find_invalid_chars(&chars, 0, &mut open_stack, &mut invalid_chars);

    if !invalid_chars.is_empty() || complete {
        return 0;
    }

    // Calculate score
    let score = open_stack
        .iter()
        .rev()
        .map(|c| match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!(),
        })
        .fold(0, |acc, s| acc * 5 + s);

    score
}

pub fn find_incomplete(lines: &[&str]) -> i64 {
    let mut scores = lines
        .iter()
        .map(|l| find_incomplete_in_line(l))
        .filter(|s| s > &0)
        .collect::<Vec<_>>();

    // Assume always odd scores...
    scores.sort();
    scores[(scores.len() / 2)]
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_1, 0, 0)]
    pub fn find_syntax_errors_in_line_test<T: AsRef<[&'static str]>>(#[case] lines: T, #[case] i: usize, #[case] expected: i64) {
        let result = find_syntax_errors_in_line(lines.as_ref()[i]);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_1, 26397)]
    #[case(TEST_DATA_2, 366027)]
    pub fn find_syntax_errors_test<T: AsRef<[&'static str]>>(#[case] lines: T, #[case] expected: i64) {
        let result = find_syntax_errors(lines.as_ref());
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_1, 0, 288957)]
    pub fn find_incomplete_in_line_test<T: AsRef<[&'static str]>>(#[case] lines: T, #[case] i: usize, #[case] expected: i64) {
        let result = find_incomplete_in_line(lines.as_ref()[i]);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_1, 288957)]
    #[case(TEST_DATA_2, 1118645287)]
    pub fn find_incomplete_test<T: AsRef<[&'static str]>>(#[case] lines: T, #[case] expected: i64) {
        let result = find_incomplete(lines.as_ref());
        assert_eq!(expected, result);
    }
}