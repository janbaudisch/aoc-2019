enum ExitCode {
    Continue,
    Halt,
}

pub struct Program {
    code: Vec<usize>,
    memory: Vec<usize>,
}

impl Program {
    pub fn run(&mut self, noun: usize, verb: usize) -> usize {
        self.memory[1] = noun;
        self.memory[2] = verb;
        self.step(0);
        self.memory[0]
    }

    pub fn reset(&mut self) {
        self.memory = self.code.clone();
    }

    fn step(&mut self, pointer: usize) {
        match self.instruction(pointer) {
            ExitCode::Continue => self.step(pointer + 4),
            ExitCode::Halt => {}
        }
    }

    fn instruction(&mut self, pointer: usize) -> ExitCode {
        let opcode = self.memory[pointer];

        if opcode == 99 {
            return ExitCode::Halt;
        }

        let left_pointer = self.memory[pointer + 1];
        let left = self.memory[left_pointer];

        let right_pointer = self.memory[pointer + 2];
        let right = self.memory[right_pointer];

        let result_pointer = self.memory[pointer + 3];

        self.memory[result_pointer] = match opcode {
            1 => left + right,
            2 => left * right,
            _ => panic!("unknown opcode: {}", opcode),
        };

        ExitCode::Continue
    }
}

impl From<String> for Program {
    fn from(string: String) -> Self {
        let code: Vec<usize> = string
            .split(',')
            .map(|x| usize::from_str_radix(x, 10).expect("error converting input"))
            .collect();

        Self {
            code: code.clone(),
            memory: code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Program;

    #[test]
    fn part_one() {
        let mut program = Program::from(String::from("1,0,0,0,99"));
        program.run(0, 0);
        assert_eq!(program.memory, [2, 0, 0, 0, 99]);

        let mut program = Program::from(String::from("2,3,0,3,99"));
        program.run(3, 0);
        assert_eq!(program.memory, [2, 3, 0, 6, 99]);

        let mut program = Program::from(String::from("2,4,4,5,99,0"));
        program.run(4, 4);
        assert_eq!(program.memory, [2, 4, 4, 5, 99, 9801]);

        let mut program = Program::from(String::from("1,1,1,4,99,5,6,0,99"));
        program.run(1, 1);
        assert_eq!(program.memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
