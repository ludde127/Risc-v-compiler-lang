use std::fmt::{Display, Formatter};
use std::fs;

//https://itnext.io/risc-v-instruction-set-cheatsheet-70961b4bbe8
trait InstructionTrait {
    //fn new(name: &str, args: &[&str]) -> Self;
    fn get_instruction_name(&self) -> &str;
    fn new(name: &str, instruction_id: &str,
           instruction_type: InstructionType,
           extra_instruction_data: Vec<ExtraInstructionData>) -> Self;
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

#[derive(Debug)]
enum ExtraInstructionData {
    Funct3(String),
    Funct7(String)
}

impl ExtraInstructionData {
    fn from_string(string: &str) -> Option<ExtraInstructionData> {

        let inside_parenthesis = {
            if string.contains("(") && string.contains(")") {
                let temp: Vec<&str> = string.split("(").collect();
                Some(temp[1].replace(")", ""))
            } else {
                None
            }
        };
        if string.contains("funct3(")  {
            Some(ExtraInstructionData::Funct3(inside_parenthesis.expect("No input in parenthesis")))
        } else if string.contains("funct7(")  {
            Some(ExtraInstructionData::Funct7(inside_parenthesis.expect("No input in parenthesis")))
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum InstructionType {
    R,
    I,
    S,
    B,
    U,
    J
}
impl InstructionType {
    fn from_string(string: &str) -> Option<InstructionType> {
        match string {
            "R" => Some(InstructionType::R),
            "I" => Some(InstructionType::I),
            "S" => Some(InstructionType::S),
            "B" => Some(InstructionType::B),
            "U" => Some(InstructionType::U),
            "J" => Some(InstructionType::J),
            _ => None
        }
    }
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
    op_code: String,
    instruction_type: InstructionType,
    extra_instruction_data: Vec<ExtraInstructionData>
}

impl InstructionTrait for Instruction {
    fn get_instruction_name(&self) -> &str {
        &self.name
    }

    fn new(name: &str, instruction_id: &str,
           instruction_type: InstructionType,
           extra_instruction_data: Vec<ExtraInstructionData>) -> Self {
        Instruction {
            name: name.parse().unwrap(),
            op_code: instruction_id.parse().unwrap(),
            instruction_type: instruction_type,
            extra_instruction_data
        }
    }

    fn from_string(string: &str) -> Self {
        let splat = string.split(" ");
        let mut name = "";
        let mut instruction_bin: &str = "";

        let mut t: Option<InstructionType> = None;
        let data:Vec<&str> = string.split(",").collect();
        let mut extra_instruction_data = vec![];
        for (i, c) in data.iter().enumerate() {
            match i {
                0 => name = c.trim(),
                1 => instruction_bin = c.trim(),
                2 => t = InstructionType::from_string(c.trim()),
                _ => extra_instruction_data.push(ExtraInstructionData::from_string(c).unwrap())
            }
        }

        Self::new(name,instruction_bin, t.unwrap(), extra_instruction_data)
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

impl Display for RiscVInstructionSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.instructions {
            write!(f, "Instruction: {}/{}, Type: {:?}, Extra: {:?}\n", i.name, i.op_code, i.instruction_type, i.extra_instruction_data);
        }
        Ok(())
    }
}