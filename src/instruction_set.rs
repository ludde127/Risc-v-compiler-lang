use std::fmt::{Display, Formatter};
use std::fs;
use crate::instructions::{Instruction, InstructionTrait, InstructionType, FilledInstruction};

pub trait InstructionSet {
    fn new() -> Self;
    fn get_instructions(&self) -> &Vec<Instruction>;
    fn add_instruction(&mut self, instruction: Instruction);

}

pub struct RiscVInstructionSet {
    instructions: Vec<Instruction>,
    reserved_registers: u8
}

impl InstructionSet for RiscVInstructionSet {
    fn new() -> Self {
        let mut instructions = vec![];
        for (i, row) in fs::read_to_string("src/instructions/risc-v.txt").expect("Could not open file").split("\n").enumerate() {
            if row.len() > 0 && i != 0 {
                instructions.push(Instruction::from_string(row));
            }
        }
        RiscVInstructionSet { instructions, reserved_registers: 32 }
    }

    fn get_instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

struct InstructionStack {
    instructions: Vec<FilledInstruction>
}

impl InstructionStack {
    fn to_binary_codes(&self) -> Vec<String> {
        let mut codes: Vec<String> = vec![];
        for instruction in &self.instructions {
            codes.push(InstructionType::format_machine_instruction(instruction).expect("Could not translate instruction"));
        }
        codes
    }
}

impl Display for RiscVInstructionSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instructions:\n");
        for i in &self.instructions {
            write!(f, "---{}\n", i);
        }
        Ok(())
    }
}