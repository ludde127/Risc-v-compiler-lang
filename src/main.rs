mod instructions;
mod io;
mod instruction_set;

use instruction_set::{RiscVInstructionSet, InstructionSet};

fn main() {
    println!("Hello, world!");
    let instructions = RiscVInstructionSet::new();
    println!("{}", instructions)
}


