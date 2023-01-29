//program memory can ADDRESSS 0x1FF + 1 (512) memory locations
//0x0FF + 1 (128) memory locations are on-chip

//2 rows of 9-bit stack pointer

//PC is 9-bits, lower 8 are stored in PCL register

use crate::nbitnumber::{u12, u9, self, NumberOperations};

pub const RESET_VECTOR: u12 = nbitnumber::NBitNumber { value: 0x00 };


pub struct ProgramMemory {
    memory: [u12; 0x200],
    stack: [u9; 0x002]
}



impl ProgramMemory {
    pub fn new() -> Self {
        ProgramMemory {
            memory: [u12::new(0); 0x200], //we only instantiate on-chip memory
            stack: [u9::new(0); 0x002] // for stack level 1 and 2
        }
    }
    
    pub fn fetch(&self, program_counter : u9) -> u12 {
         self.memory[program_counter.as_usize()]
    }

    pub fn flash(&mut self, new_program: [u12; 0x200])  {
        self.memory = new_program;
    }

}
