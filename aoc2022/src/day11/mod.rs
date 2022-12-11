#[cfg(test)] mod data;

#[derive(Debug)]
pub struct Monkey {
    pub starting_items: Vec<i64>,
    pub operation: (OperationVariable, OperationVariable, Operation, OperationVariable), // X = Y ? Z
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
    New,
    Number(i64)
}

impl From<&str> for OperationVariable {
    fn from(v: &str) -> Self {
        match v {
            "old" => OperationVariable::Old,
            "new" => OperationVariable::New,
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
            starting_items: lines[0]
                .split_ascii_whitespace()
                .skip(2)
                .map(|s| {
                    let s = if s.ends_with(',') { &s[..s.len() - 1] } else { s };
                    s.parse::<i64>().unwrap()
                })
                .collect::<Vec<i64>>(),
            operation: match lines[1].split_ascii_whitespace().skip(1).collect::<Vec<_>>().as_slice() {
                [v1, _, v2, op, v3] => (
                    OperationVariable::from(*v1),
                    OperationVariable::from(*v2),
                    Operation::from(*op),
                    OperationVariable::from(*v3),
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

        println!("{monkey:?}");
        monkeys.push(monkey);
    }

    monkeys
}

pub fn calculate_monkey_business(monkeys: &[Monkey]) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 10605)]
    #[case(TEST_DATA_1, 0)]
    fn calculate_monkey_business_test<const N: usize>(#[case] raw_data: [&str; N], #[case] expected: i64) {
        let data = parse_monkeys(&raw_data);
        let result = calculate_monkey_business(&data);

        assert_eq!(expected, result);
    }
}