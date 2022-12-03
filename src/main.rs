mod instructions;
mod io;

use instructions::{RiscVInstructionSet, InstructionSet};

fn main() {
    println!("Hello, world!");
    let instructions = RiscVInstructionSet::new();
    println!("{}", instructions)
}


