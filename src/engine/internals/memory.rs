use super::svm_error::SVMError;
use super::extensions::MemoryValue;
use super::svm_constants::MEMORY_SIZE_MAX;

pub struct Memory {
    memory: [u16; MEMORY_SIZE_MAX],
}

impl Memory {
    pub fn new(data: [u16; MEMORY_SIZE_MAX]) -> Memory {
        Memory {
            memory: data.clone(),
        }
    }

    pub fn store_memory(&mut self, address: u16, value: u16) -> Result<(), SVMError> {
        if !address.is_valid_memory_address() {
            Err(SVMError::InvalidMemory)
        } else {
            let address_value = address as usize;
            self.memory[address_value] = value;
            Ok(())
        }
    }

    pub fn load_memory(&self, address: u16) -> Result<u16, SVMError> {
        if !address.is_valid_memory_address() {
            Err(SVMError::InvalidMemory)
        } else {
            let address_value = address as usize;
            Ok(self.memory[address_value])
        }
    }
}
