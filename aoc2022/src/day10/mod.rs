#[cfg(test)] mod data;

pub enum Instruction {
    NOP,
    AddX(i32)
}

pub struct Program<'a> {
    cycle: usize,
    register_x: i32,
    instructions: &'a [Instruction],
    instruction_idx: usize,
    current_instruction: Option<(i32, i32)>,
}

impl<'a> Program<'a> {
    pub fn from_instructions(instructions: &'a [Instruction]) -> Program {
        Program {
            cycle: 1usize,
            register_x: 1i32,
            instructions,
            instruction_idx: 0usize,
            current_instruction: None,
        }
    }

    fn run_cycle(&mut self) {
        if self.has_ended() {
            return;
        }

        if self.current_instruction.is_none() {
            self.load_instruction();
        }

        match self.current_instruction {
            Some((1, add)) => {
                self.register_x += add;
                self.current_instruction = None;
            },
            Some((c, add)) => {
                self.current_instruction = Some((c - 1, add));
            },
            _ => {
                // Do nothing
            }
        };

        self.cycle += 1;
    }

    fn load_instruction(&mut self) {
        let Some(instruction) = self.instructions.get(self.instruction_idx) else {
            return;
        };

        self.instruction_idx += 1;

        self.current_instruction = match instruction {
            Instruction::NOP => Some((1, 0i32)),
            Instruction::AddX(add) => Some((2, *add))
        };
    }

    fn has_ended(&self) -> bool {
        self.instruction_idx.ge(&self.instructions.len())
            && self.current_instruction.is_none()
    }
}

impl<'a> Iterator for Program<'a> {
    type Item = (usize, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_ended() {
            return None;
        }

        self.run_cycle();
        Some((self.cycle, self.register_x))
    }
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

pub fn execute_instructions_2(instructions: &[Instruction], cycles: &[usize]) -> i32 {
    let mut cycle_index = 0usize;
    let mut cycle_sum = 0i32;

    let program = Program::from_instructions(instructions);

    // Run instructions
    for (cycle, reg_x) in program {
        // Update tracked cycles
        if cycles
            .get(cycle_index)
            .map(|c| c.eq(&cycle))
            .unwrap_or_default() {
            cycle_sum += (cycle as i32) * reg_x;
            cycle_index += 1;
        }
    }

    cycle_sum
}

pub fn emulate_crt_display(instructions: &[Instruction]) {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;

    let mut line = ['.'; WIDTH];

    // . = Dark, # = Lit

    let program = Program::from_instructions(instructions);

    // Run instructions
    for (mut cycle, reg_x) in [(1, 1)].into_iter().chain(program) {
        cycle -= 1; // Fix offset

        let pixel_idx = (cycle % WIDTH) as i32;

        let char_to_draw = [reg_x - 1, reg_x, reg_x + 1]
            .iter()
            .filter(|r| r.eq(&&pixel_idx))
            .next()
            .map(|_| '#')
            .unwrap_or('.');

        // Update pixel
        line[pixel_idx as usize] = char_to_draw;

        if pixel_idx as usize == (WIDTH - 1) {
            println!("{}", String::from_iter(line));
        }
    }
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
        let result = execute_instructions_2(&data, &cycles);

        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(TEST_DATA_0)]
    #[case(TEST_DATA_1)] // RZEKEFHA
    fn emulate_crt_display_test<const N: usize>(#[case] data: [Instruction; N]) {
        let result = emulate_crt_display(&data);
    }
}