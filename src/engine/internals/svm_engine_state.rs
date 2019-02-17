use super::memory::Memory;
use super::registers::Registers;

pub struct SVMEngineState {
    memory: Memory,
    registers: Registers,

    stack: Vec<u16>,
}