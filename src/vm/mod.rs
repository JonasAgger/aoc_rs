pub mod memory;

use std::sync::mpsc::{channel, Receiver, Sender};

use self::memory::Memory;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OpCode {
    Addition(OpCodeArgument, OpCodeArgument, OpCodeArgument),
    Multiplication(OpCodeArgument, OpCodeArgument, OpCodeArgument),
    Input(OpCodeArgument),
    Output(OpCodeArgument),
    JumpIfTrue(OpCodeArgument, OpCodeArgument),
    JumpIfFalse(OpCodeArgument, OpCodeArgument),
    LessThan(OpCodeArgument, OpCodeArgument, OpCodeArgument),
    Equals(OpCodeArgument, OpCodeArgument, OpCodeArgument),
    RelativeBase(OpCodeArgument),
    Halt,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum OpCodeArgument {
    Position(i64),
    Immediate(i64),
    Relative(i64),
}

impl OpCode {
    pub fn get_instruction(memory: &Memory, instruction_pointer: usize) -> Self {
        let ins = memory.get_value(instruction_pointer);
        let instruction = ins % 100;

        match instruction {
            1 => Self::Addition(
                Self::get_argument(ins, memory.get_value(instruction_pointer + 1), 1),
                Self::get_argument(ins, memory.get_value(instruction_pointer + 2), 2),
                Self::get_argument(ins, memory.get_value(instruction_pointer + 3), 3),
            ),
            2 => Self::Multiplication(
                Self::get_argument(ins, memory.get_value(instruction_pointer + 1), 1),
                Self::get_argument(ins, memory.get_value(instruction_pointer + 2), 2),
                Self::get_argument(ins, memory.get_value(instruction_pointer + 3), 3),
            ),
            3 => Self::Input(Self::get_argument(
                ins,
                memory.get_value(instruction_pointer + 1),
                1,
            )),
            4 => Self::Output(Self::get_argument(
                ins,
                memory.get_value(instruction_pointer + 1),
                1,
            )),
            5 => Self::JumpIfTrue(
                Self::get_argument(ins, memory.get_value(instruction_pointer + 1), 1),
                Self::get_argument(ins, memory.get_value(instruction_pointer + 2), 2),
            ),
            6 => Self::JumpIfFalse(
                Self::get_argument(ins, memory.get_value(instruction_pointer + 1), 1),
                Self::get_argument(ins, memory.get_value(instruction_pointer + 2), 2),
            ),
            7 => Self::LessThan(
                Self::get_argument(ins, memory.get_value(instruction_pointer + 1), 1),
                Self::get_argument(ins, memory.get_value(instruction_pointer + 2), 2),
                Self::get_argument(ins, memory.get_value(instruction_pointer + 3), 3),
            ),
            8 => Self::Equals(
                Self::get_argument(ins, memory.get_value(instruction_pointer + 1), 1),
                Self::get_argument(ins, memory.get_value(instruction_pointer + 2), 2),
                Self::get_argument(ins, memory.get_value(instruction_pointer + 3), 3),
            ),
            9 => Self::RelativeBase(Self::get_argument(
                ins,
                memory.get_value(instruction_pointer + 1),
                1,
            )),
            99 => Self::Halt,
            _ => panic!("reached non valid instruction: {}", instruction),
        }
    }

    pub fn size(&self) -> usize {
        match self {
            OpCode::Addition(_, _, _) => 4,
            OpCode::Multiplication(_, _, _) => 4,
            OpCode::Halt => 0,
            OpCode::Input(_) => 2,
            OpCode::Output(_) => 2,
            OpCode::LessThan(_, _, _) => 4,
            OpCode::Equals(_, _, _) => 4,
            OpCode::JumpIfFalse(_, _) => 3,
            OpCode::JumpIfTrue(_, _) => 3,
            OpCode::RelativeBase(_) => 2,
        }
    }

    fn get_argument(instruction: i64, arg: i64, arg_index: i64) -> OpCodeArgument {
        let val = 10i64.pow(arg_index as u32 + 1);
        let reducer = 10i64.pow(arg_index as u32 + 2);

        if instruction > val && (instruction % reducer) / val == 1 {
            return OpCodeArgument::Immediate(arg);
        } else if instruction > val && (instruction % reducer) / val == 2 {
            return OpCodeArgument::Relative(arg);
        }

        OpCodeArgument::Position(arg)
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum IoMode {
    Console,
    Channels,
}

pub struct VM {
    memory: Memory,
    instruction_pointer: usize,
    relative_base: i64,
    io_mode: IoMode,
    input_sender: Sender<i64>,
    input: Receiver<i64>,
    output: Sender<i64>,
    output_receiver: Option<Receiver<i64>>,
}

impl VM {
    pub fn new(instructions: Vec<i64>) -> Self {
        let (tx, rx) = channel();
        let (tx2, rx2) = channel();
        Self {
            memory: Memory::new(instructions),
            instruction_pointer: 0,
            relative_base: 0,
            io_mode: IoMode::Console,
            input_sender: tx,
            input: rx,
            output: tx2,
            output_receiver: Some(rx2),
        }
    }

    pub fn use_channels(&mut self) -> (Sender<i64>, Receiver<i64>) {
        if self.io_mode == IoMode::Channels {
            panic!("already in Channel IoMode!");
        }
        self.io_mode = IoMode::Channels;
        (
            self.input_sender.clone(),
            self.output_receiver.take().unwrap(),
        )
    }

    pub fn reset(&mut self) {
        self.instruction_pointer = 0;
    }

    pub fn execute(&mut self) {
        loop {
            let instruction = self.get_instruction();

            match instruction {
                // ARITH
                OpCode::Addition(arg1, arg2, result) => {
                    let add_result = self.memory.get(arg1) + self.memory.get(arg2);
                    self.memory.set(result, add_result);
                }
                OpCode::Multiplication(arg1, arg2, result) => {
                    let mul_result = self.memory.get(arg1) * self.memory.get(arg2);
                    self.memory.set(result, mul_result);
                }

                // CMP
                OpCode::JumpIfTrue(arg1, arg2) => {
                    if self.memory.get(arg1) != 0 {
                        self.instruction_pointer = self.memory.get(arg2) as usize;
                    }
                }
                OpCode::JumpIfFalse(arg1, arg2) => {
                    if self.memory.get(arg1) == 0 {
                        self.instruction_pointer = self.memory.get(arg2) as usize;
                    }
                }
                OpCode::LessThan(arg1, arg2, result) => {
                    if self.memory.get(arg1) < self.memory.get(arg2) {
                        self.memory.set(result, 1);
                    } else {
                        self.memory.set(result, 0);
                    }
                }
                OpCode::Equals(arg1, arg2, result) => {
                    if self.memory.get(arg1) == self.memory.get(arg2) {
                        self.memory.set(result, 1);
                    } else {
                        self.memory.set(result, 0);
                    }
                }

                // OTHER
                OpCode::RelativeBase(arg) => {
                    let diff = self.memory.get(arg);
                    self.memory.add_relative_base(diff);
                }

                // IO
                OpCode::Input(arg) => self.input(arg),
                OpCode::Output(arg) => self.output(arg),
                OpCode::Halt => return (),
            }
        }
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    fn get_instruction(&mut self) -> OpCode {
        let instruction = OpCode::get_instruction(&self.memory, self.instruction_pointer);
        self.instruction_pointer += instruction.size();

        instruction
    }

    fn input(&mut self, arg: OpCodeArgument) {
        let value = match self.io_mode {
            IoMode::Console => {
                {
                    use std::io::Write;
                    let mut out = std::io::stdout().lock();
                    write!(out, "input: ").unwrap();
                    out.flush().unwrap();
                }

                let line = std::io::stdin().lines().next().unwrap().unwrap();
                line.parse().unwrap()
            }
            IoMode::Channels => self.input.recv().unwrap(),
        };

        self.memory.set(arg, value);
    }

    fn output(&self, arg: OpCodeArgument) {
        let val = self.memory.get(arg);

        match self.io_mode {
            IoMode::Console => {
                use std::io::Write;
                let mut out = std::io::stdout().lock();
                writeln!(out, "{}", val).unwrap();
                out.flush().unwrap();
            }
            IoMode::Channels => self.output.send(val).unwrap(),
        }
    }
}

pub fn last(recv: &Receiver<i64>) -> i64 {
    let mut result = 0;
    while let Ok(val) = recv.try_recv() {
        result = val;
    }

    result
}
