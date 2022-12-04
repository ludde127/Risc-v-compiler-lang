use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;

//https://itnext.io/risc-v-instruction-set-cheatsheet-70961b4bbe8
pub trait InstructionTrait {
    //fn new(name: &str, args: &[&str]) -> Self;
    fn get_instruction_name(&self) -> &str;
    fn new(name: &str, instruction_id: &str,
           instruction_type: InstructionType,
           extra_instruction_data: Vec<ExtraInstructionData>) -> Self;
    fn from_string(string: &str) -> Self;
}

pub enum Input {
    Register(u16), // Integer is the registers numbering
    ImmediateValue(u32) // Value of immediate val.
}

#[derive(Debug)]
pub enum InputType {
    InputRegister,
    DestinationRegister,
    ImmediateValue12,
    ImmediateValue20
}

#[derive(Debug)]
pub enum ExtraInstructionData {
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
pub enum InstructionType {
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

    pub fn input_types(instruction_type: &InstructionType) -> Vec<InputType> {
        use InputType::*;
        use InstructionType::*;
        match instruction_type {
            R => vec![DestinationRegister, InputRegister, InputRegister],
            _ => todo!()
        }
    }

    pub fn format_machine_instruction(instruction: &FilledInstruction) -> Option<String> {
        use InstructionType::*;
        let result = match instruction.instruction.instruction_type {
            R => {
                let mut code: String = String::new();
                let funct7 = instruction.instruction.extra_instruction_data.iter().find(|i| {match i {ExtraInstructionData::Funct7(_) => true, _ => false}}).expect("Funct7 not found for R-type");
                if let ExtraInstructionData::Funct7(s) = funct7 {
                    code += s;
                };

                todo!();
                Some(code)
            },
            _ => None
        };

        // If the resulting code has incorrect size give None
        if result.is_some() && result.unwrap().len() != 32 {
            None
        } else {
            result
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

pub struct Instruction {
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

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Instruction: {}/{}, Type: {:?}, Extra: {:?}\n",
               self.name, self.op_code, self.instruction_type, self.extra_instruction_data).expect("Could not print instruction");
        Ok(())
    }
}

struct FilledInstruction {
    instruction: Instruction,
    data: Vec<Input>
}

impl FilledInstruction {
    fn new(instruction: Instruction, data: Vec<Input>) -> Self {
        Self { instruction, data }
    }

    fn args_and_data(&self) -> Vec<(&InputType, &Input)> {
        let mut vector = vec![];
        use InputType::*;
        use Input::*;
        for (_type, data) in InstructionType::input_types(&self.instruction.instruction_type).iter().zip(&self.data) {
            match _type {
                DestinationRegister => assert!(match data {
                    Register(_) => true,
                    _ => false
                }),
                InputRegister => assert!(match data {
                    Register(_) => true,
                    _ => false
                }),
                ImmediateValue12 => assert!(match data {
                    ImmediateValue(_) => true,
                    _ => false
                }),
                ImmediateValue20 => assert!(match data {
                    ImmediateValue(_) => true,
                    _ => false
                }),
                _ => todo!()
            }
            vector.push((_type, data));
        }
        vector
    }
}