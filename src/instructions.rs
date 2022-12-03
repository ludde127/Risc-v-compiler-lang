use std::fmt::{Display, Formatter};
use std::fs;

//https://itnext.io/risc-v-instruction-set-cheatsheet-70961b4bbe8
trait InstructionTrait {
    //fn new(name: &str, args: &[&str]) -> Self;
    fn get_instruction_name(&self) -> &str;
    fn new(name: &str, instruction_id: &str, instruction_type: InstructionType) -> Self;
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
    instruction_type: InstructionType
}

impl InstructionTrait for Instruction {
    fn get_instruction_name(&self) -> &str {
        &self.name
    }

    fn new(name: &str, instruction_id: &str, instruction_type: InstructionType) -> Self {
        Instruction {
            name: name.parse().unwrap(),
            op_code: instruction_id.parse().unwrap(),
            instruction_type: instruction_type
        }
    }

    fn from_string(string: &str) -> Self {
        let splat = string.split(" ");
        let mut name = "";
        let mut instruction_bin: &str = "";
        let mut t: Option<InstructionType> = None;
        for (i, item) in splat.enumerate() {
            if i == 1 {
                t = InstructionType::from_string(item);
            } else if i == 0 {
                let splat: Vec<&str> = item.split(":").collect();
                name = splat[0].trim();
                instruction_bin = splat[1].trim();

            } else {
                panic!("TO MANY COLUMNS")
            }

        }
        Self::new(name,instruction_bin, t.unwrap())
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
            if row.len() > 0 {
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
            write!(f, "Instruction: {}/{}, Type: {:?}\n", i.name, i.op_code, i.instruction_type);
        }
        Ok(())
    }
}