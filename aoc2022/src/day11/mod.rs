#[cfg(test)] mod data;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Monkey {
    pub items: VecDeque<i64>,
    pub operation: (OperationVariable, Operation, OperationVariable), // Y op Z
    pub test: TestOperation,
    pub test_result: (usize, usize), // True, False
}

#[derive(Debug)]
pub enum Operation {
    Multiply,
    Add
}

impl From<&str> for Operation {
    fn from(v: &str) -> Self {
        match v {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => unimplemented!("Can't parse {v} as Operation")
        }
    }
}

#[derive(Debug)]
pub enum TestOperation {
    DivisibleBy(i64)
}

#[derive(Debug)]
pub enum OperationVariable {
    Old,
    Number(i64)
}

impl From<&str> for OperationVariable {
    fn from(v: &str) -> Self {
        match v {
            "old" => OperationVariable::Old,
            d if d.chars().all(char::is_numeric) => OperationVariable::Number(d.parse::<i64>().unwrap()),
            _ => unimplemented!("Can't parse {v} as OperationVariable")
        }
    }
}

pub fn parse_monkeys(data: &[&str]) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    for monkey_data in data {
        let lines = monkey_data
            .lines()
            .skip(1)
            .map(|l| l.trim())
            .collect::<Vec<_>>();

        let monkey = Monkey {
            items: lines[0]
                .split_ascii_whitespace()
                .skip(2)
                .map(|s| {
                    let s = if s.ends_with(',') { &s[..s.len() - 1] } else { s };
                    s.parse::<i64>().unwrap()
                })
                .collect::<VecDeque<i64>>(),
            operation: match lines[1].split_ascii_whitespace().skip(3).collect::<Vec<_>>().as_slice() {
                [v1, op, v2] => (
                    OperationVariable::from(*v1),
                    Operation::from(*op),
                    OperationVariable::from(*v2),
                ),
                _ => unimplemented!("Can't parse operation")
            },
            test: {
                let num = lines[2]
                    .split_ascii_whitespace()
                    .rev()
                    .take(1)
                    .map(|s| s.parse::<i64>().unwrap())
                    .next()
                    .unwrap();

                TestOperation::DivisibleBy(num)
            },
            test_result: {
                let if_true = lines[3]
                    .split_ascii_whitespace()
                    .rev()
                    .take(1)
                    .map(|s| s.parse::<usize>().unwrap())
                    .next()
                    .unwrap();

                let if_false = lines[4]
                    .split_ascii_whitespace()
                    .rev()
                    .take(1)
                    .map(|s| s.parse::<usize>().unwrap())
                    .next()
                    .unwrap();

                (if_true, if_false)
            }
        };

        monkeys.push(monkey);
    }

    monkeys
}

pub fn calculate_monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, division: i64) -> i64 {
    let mut inspection_counts = vec![0i64; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            // Consume items
            while let Some(mut worry_level) = monkeys[i].items.pop_front() {
                let monkey = &monkeys[i];

                // Calculate current worry level
                worry_level = resolve_operation(&monkey.operation, worry_level);

                // Calculate thrown worry level
                if division != 1 {
                    worry_level = ((worry_level as f64) / (division as f64)).floor() as i64;
                }

                // Run test
                let (throw_if_true, throw_if_false) = monkey.test_result;
                let test_passes = match monkey.test {
                    TestOperation::DivisibleBy(div) => worry_level % div == 0
                };

                // Throw new level to other monkey
                if test_passes {
                    monkeys[throw_if_true].items.push_back(worry_level);
                } else {
                    monkeys[throw_if_false].items.push_back(worry_level);
                }

                // Update inspection count
                inspection_counts[i] += 1;
            }
        }
    }

    //monkeys
    //    .iter()
    //    .enumerate()
    //    .for_each(|(i, m)| {
    //        println!("{m:?} ({} times)", inspection_counts[i]);
    //    });

    inspection_counts.sort();

    inspection_counts
        .iter()
        .rev()
        .take(2)
        .map(|i| *i)
        .product()
}

pub fn resolve_operation((op_v1, op, op_v2): &(OperationVariable, Operation, OperationVariable), old_level: i64) -> i64 {
    let v1 = match op_v1 {
        OperationVariable::Number(n) => *n,
        OperationVariable::Old => old_level
    };

    let v2 = match op_v2 {
        OperationVariable::Number(n) => *n,
        OperationVariable::Old => old_level
    };

    match op {
        Operation::Add => v1 + v2,
        Operation::Multiply => v1 * v2
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 20, 3, 10605)]
    #[case(TEST_DATA_1, 20, 3, 110220)]
    #[case(TEST_DATA_0, 1000, 1, 2713310158)]
    #[case(TEST_DATA_1, 1000, 1, 0)]
    fn calculate_monkey_business_test<const N: usize>(#[case] raw_data: [&str; N], #[case] rounds: usize, #[case] division: i64, #[case] expected: i64) {
        let data = parse_monkeys(&raw_data);
        let result = calculate_monkey_business(data, rounds, division);

        assert_eq!(expected, result);
    }
}