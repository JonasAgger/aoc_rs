pub mod day_02;

enum OpCode {
    Addition(OpCodeArgument, OpCodeArgument, OpCodeArgument),
    Multiplication(OpCodeArgument, OpCodeArgument, OpCodeArgument),
    Halt,
}

enum OpCodeArgument {
    Position(i64),
}

impl OpCode {
    pub fn get_instruction(instructions: &[i64], instruction_pointer: usize) -> Self {
        match instructions[instruction_pointer..] {
            [1, arg1, arg2, arg3, ..] => Self::Addition(
                OpCodeArgument::Position(arg1),
                OpCodeArgument::Position(arg2),
                OpCodeArgument::Position(arg3),
            ),
            [2, arg1, arg2, arg3, ..] => Self::Multiplication(
                OpCodeArgument::Position(arg1),
                OpCodeArgument::Position(arg2),
                OpCodeArgument::Position(arg3),
            ),
            [99, ..] => Self::Halt,
            _ => panic!("reached non valid instruction"),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            OpCode::Addition(_, _, _) => 4,
            OpCode::Multiplication(_, _, _) => 4,
            OpCode::Halt => 0,
        }
    }
}

struct VM {
    instructions: Vec<i64>,
    instruction_pointer: usize,
}

impl VM {
    pub fn new(instructions: Vec<i64>) -> Self {
        Self {
            instructions,
            instruction_pointer: 0,
        }
    }

    pub fn execute(&mut self) {
        loop {
            let instruction = self.get_instruction();
            match instruction {
                OpCode::Addition(arg1, arg2, result) => {
                    let add_result = self.get(arg1) + self.get(arg2);

                    match result {
                        OpCodeArgument::Position(pos) => {
                            self.instructions[pos as usize] = add_result
                        }
                    }
                }
                OpCode::Multiplication(arg1, arg2, result) => {
                    let add_result = self.get(arg1) * self.get(arg2);

                    match result {
                        OpCodeArgument::Position(pos) => {
                            self.instructions[pos as usize] = add_result
                        }
                    }
                }
                OpCode::Halt => return (),
            }
        }
    }

    pub fn instructions(&self) -> &[i64] {
        &self.instructions
    }

    pub fn get_value_at(&self, index: usize) -> i64 {
        self.instructions[index]
    }

    fn get(&self, argument: OpCodeArgument) -> i64 {
        match argument {
            OpCodeArgument::Position(pos) => self.instructions[pos as usize],
        }
    }

    fn get_instruction(&mut self) -> OpCode {
        let instruction =
            OpCode::get_instruction(self.instructions.as_slice(), self.instruction_pointer);
        self.instruction_pointer += instruction.size();

        instruction
    }
}
