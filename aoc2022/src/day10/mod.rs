#[cfg(test)] mod data;

pub enum Instruction {
    NOP,
    AddX(i32)
}

pub fn execute_instructions(instructions: &[Instruction], cycles: &[usize]) -> i32 {
    let mut cycle = 1usize;
    let mut register_x = 1i32;

    let mut cycle_index = 0usize;
    let mut cycle_sum = 0i32;

    // Run instructions
    for instruction in instructions {
        let (cycle_count, add_x) = match instruction {
            Instruction::NOP => (1, 0i32),
            Instruction::AddX(add) => (2, *add)
        };

        for _ in 0..cycle_count {
            // Update tracked cycles
            if cycles
                .get(cycle_index)
                .map(|c| c.eq(&cycle))
                .unwrap_or_default() {
                cycle_sum += (cycle as i32) * register_x;
                cycle_index += 1;
            }

            cycle += 1;
        }

        register_x += add_x;
    }

    cycle_sum
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::{*, data::*};

    #[rstest]
    #[case(TEST_DATA_0, 13140)]
    #[case(TEST_DATA_1, 14820)]
    fn execute_instructions_test<const N: usize>(#[case] data: [Instruction; N], #[case] expected: i32) {
        let cycles = [20, 60, 100, 140, 180, 220];
        let result = execute_instructions(&data, &cycles);

        assert_eq!(expected, result);
    }
}