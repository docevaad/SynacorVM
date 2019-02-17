use super::instruction_pointer::InstructionPointer;
use super::memory::{MemoryArray,Memory};
use super::registers::Registers;

pub struct SVMEngineState {
    pub instruction_pointer: InstructionPointer,
    pub registers: Registers,
    pub memory: Memory,
    pub stack: Vec<u16>,
}

impl SVMEngineState {
    pub fn new(program_data: MemoryArray) -> SVMEngineState {
        SVMEngineState {
            instruction_pointer: InstructionPointer::new(),
            registers: Registers::new(),
            memory: Memory::new(program_data),
            stack: Vec::new(),
        }
    }
}