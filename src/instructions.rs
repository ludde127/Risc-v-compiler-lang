use std::fmt::{Display, Formatter};
use std::fs;

//https://itnext.io/risc-v-instruction-set-cheatsheet-70961b4bbe8
trait InstructionTrait {
    //fn new(name: &str, args: &[&str]) -> Self;
    fn get_instruction_name(&self) -> &str;
    fn new(name: &str, arg_mappings: Vec<InputType>) -> Self;
    fn from_string(string: &str) -> Self;
}

enum Input {
    Register(u16), // Integer is the registers numbering
    ImmediateValue(u32) // Value of immediate val.
}
#[derive(Debug)]
enum InputType {
    InputRegister,
    DestinationRegister,
    ImmediateValue12,
    ImmediateValue20
}

impl InputType {
    fn from_string(string: &str) -> Option<InputType> {
        if string.contains("rd") {
            Some(InputType::DestinationRegister)
        } else if string.contains("rs") {
            Some(InputType::DestinationRegister)
        } else if string.contains("imm12") {
            Some(InputType::ImmediateValue12)
        } else if string.contains("imm20") {
            Some(InputType::ImmediateValue20)
        } else {
            None
        }
    }
}

struct Instruction {
    name: String,
    nbr_of_args: u8, // How many arguments does the instruction take? B R0 Takes one argument.
    args: Vec<InputType>
}

impl InstructionTrait for Instruction {
    fn get_instruction_name(&self) -> &str {
        &self.name
    }

    fn new(name: &str, args: Vec<InputType>) -> Self {
        Instruction {
            name: name.parse().unwrap(),
            nbr_of_args: args.len() as u8,
            args,
        }
    }

    fn from_string(string: &str) -> Self {
        let mut args = vec![];
        let splat = string.split(" ");
        let mut name = "";
        for (i, item) in splat.enumerate() {
            if i != 0 {
                args.push(InputType::from_string(item).expect(&*format!("Could not parse input type {item}", item=item)));
            } else {
                name = item;
            }

        }
        Self::new(name, args)
    }
}

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
        for row in fs::read_to_string("src/instructions/risc-v.txt").expect("Could not open file").split("\n") {
            instructions.push(Instruction::from_string(row));
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

impl Display for RiscVInstructionSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.instructions {
            write!(f, "Instruction: {}, args: {:?}\n", i.name, i.args);
        }
        Ok(())
    }
}