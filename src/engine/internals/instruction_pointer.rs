use super::extensions::MemoryValue;
use super::memory::Memory;
use super::svm_error::SVMError;

pub struct InstructionPointer {
    ip: u16,
}

impl InstructionPointer {
    pub fn new() -> InstructionPointer {
        InstructionPointer {
            ip: 0,
        }
    }

    pub fn set_ip(&mut self, address: u16) -> Result<(), SVMError> {
        if !address.is_valid_memory_address() {
            Err(SVMError::InvalidMemory)
        } else {
            self.ip = address;
            Ok(())
        }
    }

    pub fn get_ip(&self) -> u16 {
        self.ip
    }

    pub fn get_next_memory_value(&mut self, memory: &Memory) -> Result<u16, SVMError> {
        self.ip += 1;
        memory.load_memory(self.ip)
    }
}